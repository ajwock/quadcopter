#![no_std]
#![no_main]

mod icm42670;
mod imu_common;
mod motor_drive;
mod motion_data;
mod utils;
mod receiver;

use esp_println::println;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Ticker};
use core::net::Ipv4Addr;
use core::str::FromStr;
use embassy_net::{
    tcp::TcpSocket,
    IpListenEndpoint,
    Ipv4Cidr,
    Runner,
    Stack,
    StackResources,
    StaticConfigV4,
};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::i2c;
use icm42670::Icm42670;
use motion_data::MotionData;
use embassy_sync::{
    signal::Signal,
    blocking_mutex::raw::CriticalSectionRawMutex,
};
use heapless::String;
use esp_wifi::{
    init,
    wifi::{
        AccessPointConfiguration,
        AuthMethod,
        Configuration,
        WifiController,
        WifiDevice,
        WifiEvent,
        WifiState,
    },
    EspWifiController,
};

use imu_common::{ImuCalibrator, ImuController};

extern crate alloc;

static IMU_START_READ: Signal<CriticalSectionRawMutex, ()> = Signal::new();
static IMU_READ_DONE: Signal<CriticalSectionRawMutex, MotionData> = Signal::new();

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[embassy_executor::task]
async fn imu_read_task(mut imu: ImuController<Icm42670<'static>>) {
    loop {
        IMU_START_READ.wait().await;
        println!("reading motiondata");
        let motion_data = imu.read_motion_data().await;
        println!("done");
        IMU_READ_DONE.signal(motion_data);
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let mut rng = esp_hal::rng::Rng::new(peripherals.RNG);

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let initted = &*mk_static!(EspWifiController<'static>, esp_wifi::init(
        timer1.timer0,
        rng.clone(),
        peripherals.RADIO_CLK,
    )
    .unwrap());

    let wifi_config = esp_wifi::wifi::Configuration::AccessPoint(
        AccessPointConfiguration {
            ssid: "esp_quad_wifi".try_into().unwrap(),
            ssid_hidden: false,
            auth_method: AuthMethod::WPA2Personal,
            password: "lol1337".try_into().unwrap(),
            ..Default::default()
        }
    );
    let (controller, interfaces) = esp_wifi::wifi::new(
        &initted,
        peripherals.WIFI,
    ).unwrap();

    let gw_ip_addr_str = "192.168.2.1";
    let gw_ip_addr = Ipv4Addr::from_str(gw_ip_addr_str).expect("failed to parse gateway ip");

    let config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(gw_ip_addr, 24),
        gateway: Some(gw_ip_addr),
        dns_servers: Default::default(),
    });

    let net_seed = rng.random() as u64;
    let (stack, runner) = embassy_net::new(
        interfaces.sta,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    spawner.spawn(connection(controller)).unwrap();
    spawner.spawn(net_task(runner)).unwrap();
    spawner.spawn(run_dhcp(stack, gw_ip_addr_str)).unwrap();

    let i2c = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default()
            .with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO10)
    .with_scl(peripherals.GPIO8)
    .into_async();

    let mut imu = Icm42670::new(i2c); 
    imu.configure().await;
    let mut calibrator = ImuCalibrator::new(imu);

    // Tick the calibrator state machine until it's done
    let mut ticker = Ticker::every(Duration::from_millis(100));
    let imuctl = loop {
        if let Some(out) = calibrator.calibration_tick().await {
            break out
        }
        ticker.next().await
    };
    // With the imu configured, put it in a task so it doesn't block
    spawner
        .spawn(imu_read_task(imuctl)).unwrap();

    let _ = spawner;
    let mut prev_motiondata = MotionData::zero();
    loop {
        IMU_START_READ.signal(());
        prev_motiondata.show();
        let motion_data = IMU_READ_DONE.wait().await;
        prev_motiondata = motion_data;
        ticker.next().await;
    }
}

#[embassy_executor::task]
async fn run_dhcp(stack: Stack<'static>, gw_ip_addr: &'static str) {
    use core::net::{Ipv4Addr, SocketAddrV4};

    use edge_dhcp::{
        io::{self, DEFAULT_SERVER_PORT},
        server::{Server, ServerOptions},
    };
    use edge_nal::UdpBind;
    use edge_nal_embassy::{Udp, UdpBuffers};

    let ip = Ipv4Addr::from_str(gw_ip_addr).expect("dhcp task failed to parse gw ip");

    let mut buf = [0u8; 1500];

    let mut gw_buf = [Ipv4Addr::UNSPECIFIED];

    let buffers = UdpBuffers::<3, 1024, 1024, 10>::new();
    let unbound_socket = Udp::new(stack, &buffers);
    let mut bound_socket = unbound_socket
        .bind(core::net::SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::UNSPECIFIED,
            DEFAULT_SERVER_PORT,
        )))
        .await
        .unwrap();

    loop {
        _ = io::server::run(
            &mut Server::<_, 64>::new_with_et(ip),
            &ServerOptions::new(ip, Some(&mut gw_buf)),
            &mut bound_socket,
            &mut buf,
        )
        .await
        .inspect_err(|e| println!("DHCP server error: {e:?}"));
        Timer::after(Duration::from_millis(500)).await;
    }
}


#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::ApStarted => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::ApStop).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = Configuration::AccessPoint(AccessPointConfiguration {
                ssid: "esp-wifi".try_into().unwrap(),
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
