#![no_std]
#![no_main]

mod imu_common;
mod motor_drive;
mod motion_data;
mod motion_data_angular;
mod orientation_tracking;
mod utils;
mod receiver;
mod delay_buf;
mod icm42670_imu;

use motion_data::DegreeFixed32;
use fixed_macro::fixed;
use orientation_tracking::OrientationTracker;
use embedded_hal_async::delay::DelayNs;
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
use icm42670::{
    Icm42670,
    DLPF,
    ODR,
    AccelConfig,
    GyroConfig,
    AccelRange,
    GyroRange,
};
use motion_data::MotionData;
use embassy_sync::{
    signal::Signal,
    mutex::Mutex,
    blocking_mutex::raw::{
        CriticalSectionRawMutex,
    },
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
use core::sync::atomic::{Ordering, AtomicBool, AtomicI8, AtomicU8, AtomicI32};
use regcomms::i2c::I2cCommsAsync;

use imu_common::{Imu, ImuCalibrator};

extern crate alloc;

#[derive(Copy, Clone, Debug)]
pub struct ControlVals {
    pub tilt_x: i8,
    pub tilt_y: i8,
    pub rot_z: i8,
    pub collective: u8,
}

pub struct Controls {
    pub tilt_x: AtomicI8,
    pub tilt_y: AtomicI8,
    pub rot_z: AtomicI8,
    pub collective: AtomicU8,
}

impl Controls {
    fn get_vals(&self) -> ControlVals {
        ControlVals {
            tilt_x: self.tilt_x.load(Ordering::Relaxed),
            tilt_y: self.tilt_y.load(Ordering::Relaxed),
            rot_z: self.rot_z.load(Ordering::Relaxed),
            collective: self.collective.load(Ordering::Relaxed),
        }
    }

    fn update(&self, vals: ControlVals) {
        self.tilt_x.store(vals.tilt_x, Ordering::Relaxed);
        self.tilt_y.store(vals.tilt_y, Ordering::Relaxed);
        self.rot_z.store(vals.rot_z, Ordering::Relaxed);
        self.collective.store(vals.collective, Ordering::Relaxed);
    }
}

static CONTROLS: Controls = Controls {
    tilt_x: AtomicI8::new(0),
    tilt_y: AtomicI8::new(0),
    rot_z: AtomicI8::new(0),
    collective: AtomicU8::new(0),
};

static CURRENT_X: AtomicI32 = AtomicI32::new(0);
static CURRENT_Y: AtomicI32 = AtomicI32::new(0);
static CURRENT_Z: AtomicI32 = AtomicI32::new(0);

static CONTROLLER_CONNECTED: Signal<CriticalSectionRawMutex, ()> = Signal::new();
static CONTROLLER_DISCONNECTED: AtomicBool = AtomicBool::new(false);

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
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
        interfaces.ap,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    spawner.spawn(manage_ap_connection(controller)).unwrap();
    spawner.spawn(net_task(runner)).unwrap();
    spawner.spawn(run_dhcp(stack, gw_ip_addr_str)).unwrap();
    spawner.spawn(manage_receiver_connection(stack, gw_ip_addr_str)).unwrap();

    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(200)).await;
    }


    CONTROLLER_CONNECTED.wait().await;
    /* IMU SETUP */
    let mut conn_led = Output::new(
        peripherals.GPIO8,
        Level::Low,
        OutputConfig::default(),
    );

    let i2c = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default()
            .with_frequency(Rate::from_khz(1000)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO10)
    .with_scl(peripherals.GPIO4)
    .into_async();

    // Pull the ad0 pin low.
    let mut ad0 = Output::new(
        peripherals.GPIO1,
        Level::Low,
        OutputConfig::default(),
    );
    ad0.set_low();
    // Pull the icm42670 'cs' pin low (it's unused for i2c)
    let mut cs = Output::new(
        peripherals.GPIO5,
        Level::High,
        OutputConfig::default(),
    );
    cs.set_high();
    

    let mut d = embassy_time::Delay;
    d.delay_us(200).await;

    let config = icm42670::Config {
        accel_config: Some(AccelConfig {
            accel_range: AccelRange::G4,
            accel_odr:   ODR::Hz1600,
            accel_dlpf:  DLPF::Bypassed,
        }),
        gyro_config: Some(GyroConfig {
            gyro_range: GyroRange::DPS2000,
            gyro_odr:   ODR::Hz1600,
            gyro_dlpf:  DLPF::Bypassed,
        }),
        fifo_config: Some(Default::default()),
    };
    let comms = i2c;
    let i2c_comms = I2cCommsAsync::new(comms)
        .with_address(0b1101000);
    let mut imu = Icm42670::new(i2c_comms, embassy_time::Delay); 
    let mut ticker = Ticker::every(Duration::from_millis(10));
    //println!("Powering on");
    println!("Configuring");
    imu.configure(config).await.unwrap();
    println!("Configured");
    imu.enable().await.unwrap();
 
    for i in 0..100 {
        let mut good_packets = 0;
        debug_println!("FIFOed packet group {} {{", i);
        /*
        while let Ok(Some(packet)) = imu.read_fifo_packet().await {
            debug_println!("{:?}", packet);
            good_packets += 1;
        }*/
        while let Ok(msg) = imu.get_motion_data_msg().await {
            debug_println!("{:?}", msg);
            good_packets += 1;
        }
        debug_println!("}}");
        println!("{i}: {good_packets} packets");
        ticker.next().await;
    }
    println!("Starting calibration");
    let mut calibrator = ImuCalibrator::<_, 1024>::new(imu);
    // Tick the calibrator state machine until it's done
    let mut imuctl = calibrator.msg_calibration().await.expect("Calibration failed");
    let gravmag = imuctl.gravity_mag();

    println!("Initializing motor pwms");
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
    let mut frontleft = ledc.channel(ledc::channel::Number::Channel0, peripherals.GPIO2);
    frontleft.configure(common_chanconfig).unwrap();
    let mut frontright = ledc.channel(ledc::channel::Number::Channel1, peripherals.GPIO7);
    frontright.configure(common_chanconfig).unwrap();
    let mut backleft = ledc.channel(ledc::channel::Number::Channel2, peripherals.GPIO3);
    backleft.configure(common_chanconfig).unwrap();
    let mut backright= ledc.channel(ledc::channel::Number::Channel3, peripherals.GPIO6);
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
    let mut motor_drive = MotorDrive::new(frontleft, frontright, backleft, backright, gravmag);
    debug_println!("Motor driver set up");

    let mut led = Output::new(
        peripherals.GPIO0,
        Level::Low,
        OutputConfig::default(),
    );

    let _ = spawner;
    imuctl.flush_msgs().await;
    let mut orientation_tracker = OrientationTracker::new(imuctl);
    let mut led_tick_reducer = 0;
    println!("Starting main loop");
    loop {
        if led_tick_reducer == 20 {
            led.toggle();
            conn_led.toggle();
            led_tick_reducer = 0;
        } else {
            led_tick_reducer += 1;
        }
        if CONTROLLER_DISCONNECTED.load(Ordering::Relaxed) {
            motor_drive.cut_motors();
            panic!("Controller disconnected");
        }
        orientation_tracker.track().await;
        let orientation = orientation_tracker.get_orientation();
        println!("Orientation: {:?}", orientation);
        CURRENT_X.store(orientation[0].to_bits(), Ordering::Relaxed);
        CURRENT_Y.store(orientation[1].to_bits(), Ordering::Relaxed);
        CURRENT_Z.store(orientation[2].to_bits(), Ordering::Relaxed);
        let control_vals = CONTROLS.get_vals();
        motor_drive.set_collective_pct(control_vals.collective);
        let tilt_ctrl = [xy_tilt_input_xlat(control_vals.tilt_x), xy_tilt_input_xlat(control_vals.tilt_y), DegreeFixed32::ZERO];
        debug_println!("Target tilt: {:?}", tilt_ctrl);
        motor_drive.set_target_tilt(tilt_ctrl);
        debug_println!("Collective: {}", control_vals.collective);
        motor_drive.attitude_correct(orientation);
        motor_drive.motor_tick();
        ticker.next().await;
    }
}

const TILT_SCALE: DegreeFixed32 = fixed!(0.2: I12F20);
fn xy_tilt_input_xlat(input: i8) -> DegreeFixed32 {
    TILT_SCALE * (input as i32)
}

#[embassy_executor::task]
async fn manage_receiver_connection(stack: Stack<'static>, gw_ip_addr: &'static str) {
    let mut rx_buffer = [0; 2048];
    let mut tx_buffer = [0; 2048];
    debug_println!("Receiver connection management up");
    loop {
        let mut sock = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
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
        CONTROLLER_CONNECTED.signal(());
        debug_println!("Got tcp connection");
        use embedded_io_async::Write;
        let mut buf = [0u8; 4];
        loop {
            match sock.read(&mut buf).await {
                Ok(0) => {
                    debug_println!("Client connection closed.");
                    CONTROLLER_DISCONNECTED.store(true, Ordering::Relaxed);
                    break
                }
                Ok(len) => {
                    if len != 4 {
                        debug_println!("Got bad packet length: {}", len);
                        continue
                    }
                    debug_println!("Got packet: {:?}", &buf[0..len]);
                    let vals = ControlVals {
                        tilt_x: buf[0] as i8,
                        tilt_y: buf[1] as i8,
                        rot_z: buf[2] as i8,
                        collective: buf[3],
                    };
                    CONTROLS.update(vals);
                    debug_println!("Updated controls: {:?}", vals);
                    let x = CURRENT_X.load(Ordering::Relaxed);
                    let y = CURRENT_Y.load(Ordering::Relaxed);
                    let z = CURRENT_Z.load(Ordering::Relaxed);
                    let mut buf = [0u8; 12];
                    buf[0..4].copy_from_slice(&x.to_be_bytes());
                    buf[4..8].copy_from_slice(&y.to_be_bytes());
                    buf[8..12].copy_from_slice(&z.to_be_bytes());
                    let _ = sock.write(&mut buf).await;
                }
                Err(e) => {
                    debug_println!("Read error in control loop: {:?}", e);
                    CONTROLLER_DISCONNECTED.store(true, Ordering::Relaxed);
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
            &mut Server::<_, 256>::new_with_et(ip),
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
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::ApStarted => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::ApStop).await;
                Timer::after(Duration::from_millis(100)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let wifi_config = esp_wifi::wifi::Configuration::AccessPoint(
                AccessPointConfiguration {
                    ssid: "esp_quad_wifi".try_into().unwrap(),
                    ssid_hidden: false,
                    auth_method: AuthMethod::WPA2Personal,
                    password: "rofl1337".try_into().unwrap(),
                    ..Default::default()
                }
            );
            controller.set_configuration(&wifi_config).unwrap();
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
