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
use esp_hal::ledc::{
    self,
    Ledc,
    LSGlobalClkSource,
    timer::TimerIFace,
    channel::ChannelIFace,
};
use esp_hal::gpio::{
    self,
    Output,
    Level,
    OutputConfig,
};
use static_cell::StaticCell;
use icm42670::Icm42670;
use motion_data::MotionData;
use embassy_sync::{
    signal::Signal,
    blocking_mutex::raw::CriticalSectionRawMutex,
};
use esp_wifi::{
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
use motor_drive::MotorDrive;
use edge_nal::UdpBind;

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
        let motion_data = imu.read_motion_data().await;
        IMU_READ_DONE.signal(motion_data);
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    /* Hal / Embassy init */
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let mut rng = esp_hal::rng::Rng::new(peripherals.RNG);

    /* CONTROLLER SETUP */

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

    spawner.spawn(manage_ap_connection(controller)).unwrap();
    spawner.spawn(net_task(runner)).unwrap();
    spawner.spawn(run_dhcp(stack, gw_ip_addr_str)).unwrap();
    spawner.spawn(manage_receiver_connection(stack, gw_ip_addr_str)).unwrap();

    /* IMU SETUP */

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
    let mut ticker = Ticker::every(Duration::from_millis(10));
    let imuctl = loop {
        if let Some(out) = calibrator.calibration_tick().await {
            break out
        }
        ticker.next().await
    };
    spawner
        .spawn(imu_read_task(imuctl)).unwrap();

    /* PWM / MOTOR DRIVER SETUP */

    debug_println!("Initializing motor pwms");
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    static LSTIMER0: StaticCell<ledc::timer::Timer<'_, ledc::LowSpeed>> = StaticCell::new();
    let lstimer0 = LSTIMER0.init(ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0));
    lstimer0
    .configure(ledc::timer::config::Config {
        duty: ledc::timer::config::Duty::Duty5Bit,
        clock_source: ledc::timer::LSClockSource::APBClk,
        frequency: Rate::from_khz(24),
    }).unwrap();
    let common_chanconfig = ledc::channel::config::Config {
        timer: lstimer0,
        duty_pct: 0,
        pin_config: ledc::channel::config::PinConfig::PushPull,
    };
    let mut frontleft = ledc.channel(ledc::channel::Number::Channel0, peripherals.GPIO0);
    frontleft.configure(common_chanconfig).unwrap();
    let mut frontright = ledc.channel(ledc::channel::Number::Channel1, peripherals.GPIO1);
    frontright.configure(common_chanconfig).unwrap();
    let mut backleft = ledc.channel(ledc::channel::Number::Channel2, peripherals.GPIO2);
    backleft.configure(common_chanconfig).unwrap();
    let mut backright= ledc.channel(ledc::channel::Number::Channel3, peripherals.GPIO3);
    backright.configure(common_chanconfig).unwrap();
    
    // Whoops, need to software rotate the craft 90 degrees to the left
    let temp = frontleft;
    let frontleft = frontright;
    let frontright = backright;
    let backright = backleft;
    let backleft = temp;
    /*
    // Whoops, need to software rotate the craft 90 degrees to the right
    let temp = frontright;
    let frontright = frontleft;
    let frontleft = backleft;
    let backleft = backright;
    let backright = temp;*/
    // 1 3
    // 0 2
    let mut motor_drive = MotorDrive::new(frontleft, frontright, backleft, backright);
    debug_println!("Motor driver set up");

    let mut led = Output::new(
        peripherals.GPIO7,
        Level::High,
        OutputConfig::default(),
    );

    let _ = spawner;
    let mut prev_motiondata = MotionData::zero();
    let mut collective_pct = 0;
    let mut collective_tick_reducer = 0;
    let mut led_tick_reducer = 0;
    loop {
        IMU_START_READ.signal(());
        prev_motiondata.show();
        if collective_pct < 70 && collective_tick_reducer == 0 {
            collective_pct += 1;
        }
        collective_tick_reducer = (collective_tick_reducer + 1) % 20;
        if led_tick_reducer == 0 {
            led.toggle()
        }
        led_tick_reducer = (led_tick_reducer + 1) % 30;
        motor_drive.set_collective_pct(collective_pct);
        motor_drive.attitude_correct(prev_motiondata);
        motor_drive.motor_tick();
        let motion_data = IMU_READ_DONE.wait().await;
        prev_motiondata = motion_data;
        ticker.next().await;
    }
}

#[embassy_executor::task]
async fn manage_receiver_connection(stack: Stack<'static>, gw_ip_addr: &'static str) {
    let mut rx_buffer = [0; 2048];
    let mut tx_buffer = [0; 2048];
    let mut sock = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    loop {
        debug_println!("Waiting for connection...");
        if let Err(e) = sock
            .accept(IpListenEndpoint {
                addr: None,
                port: 4200,
            })
                .await {
            debug_println!("Socket connection error in control loop, continuing: {:?}", e);
            continue
        }
        use embedded_io_async::Write;
        let mut buf = [0u8; 1024];
        loop {
            match sock.read(&mut buf).await {
                Ok(0) => {
                    debug_println!("Client connection closed.");
                    break
                }
                Ok(len) => {
                    debug_println!("Got packet: {:?}", &buf[0..len]);
                }
                Err(e) => {
                    debug_println!("Read error in control loop: {:?}", e);
                    break
                }
            }
        }
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
        .inspect_err(|e| debug_println!("DHCP server error: {e:?}"));
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task(pool_size=10)]
async fn manage_ap_connection(mut controller: WifiController<'static>) {
    debug_println!("start connection task");
    debug_println!("Device capabilities: {:?}", controller.capabilities());
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
            debug_println!("Starting wifi");
            controller.start_async().await.unwrap();
            debug_println!("Wifi started!");
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
