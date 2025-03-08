#![no_std]
#![no_main]

mod mpu6050;
mod motor_drive;

use motor_drive::MotorDrive;
use mpu6050::Mpu6050;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::timer::PeriodicTimer;
use esp_hal::Blocking;
use esp_hal::i2c;
use esp_println::println;
use esp_hal::ledc;
use esp_hal::ledc::{
    Ledc,
    LSGlobalClkSource,
    timer::TimerIFace,
    channel::ChannelIFace,
};

use core::cell::RefCell;
use critical_section::Mutex;
use esp_hal::handler;
use static_cell::StaticCell;

extern crate alloc;

struct TopState {
    mpu: Mpu6050<'static, Blocking>,
    motors: MotorDrive,
    periodic_timer: PeriodicTimer<'static, Blocking>,
}


static MPU_6050: Mutex<RefCell<Option<Mpu6050<'static, Blocking>>>> = Mutex::new(RefCell::new(None));
static MOTOR_DRIVE: Mutex<RefCell<Option<MotorDrive>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<PeriodicTimer<'static, Blocking>>>> = Mutex::new(RefCell::new(None));

#[handler]
fn timed_interrupt_handler() {
    critical_section::with(|cs| {
        let mut mpu_borrow = MPU_6050.borrow_ref_mut(cs);
        let mpu_ref = mpu_borrow
            .as_mut()
            .unwrap();
        let motion_data = mpu_ref.read_motion_data();
        motion_data.show();
        TIMER.borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
    });
}

#[main]
fn main() -> ! {
    // generator version: 0.3.1

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let i2c = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default()
            .with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO0)
    .with_scl(peripherals.GPIO1);

    let mut mpu = Mpu6050::new(i2c);

    println!("Configuring mpu 6050");
    mpu.configure_mpu_6050();
    println!("mpu 6050 configured");

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
    let mut pwm0 = ledc.channel(ledc::channel::Number::Channel0, peripherals.GPIO2);
    pwm0.configure(common_chanconfig).unwrap();
    let mut pwm1 = ledc.channel(ledc::channel::Number::Channel1, peripherals.GPIO3);
    pwm1.configure(common_chanconfig).unwrap();
    let mut pwm2 = ledc.channel(ledc::channel::Number::Channel2, peripherals.GPIO4);
    pwm2.configure(common_chanconfig).unwrap();
    let mut pwm3 = ledc.channel(ledc::channel::Number::Channel3, peripherals.GPIO5);
    pwm3.configure(common_chanconfig).unwrap();

    let mut motor_drive = MotorDrive::new(pwm0, pwm1, pwm2, pwm3);
    println!("Motor pwms initialized");


    println!("Initializing periodic timer interrupt");
    let timg1 = TimerGroup::new(peripherals.TIMG1);
    let mut ptimer = PeriodicTimer::new(timg1.timer0);
    ptimer.set_interrupt_handler(timed_interrupt_handler);
    ptimer.enable_interrupt(true);
    ptimer.start(Duration::from_millis(100))
        .expect("Failed to start periodic timer1");
    critical_section::with(|cs| {
        MPU_6050.borrow_ref_mut(cs).replace(mpu);
        MOTOR_DRIVE.borrow_ref_mut(cs).replace(motor_drive);
        TIMER.borrow_ref_mut(cs).replace(ptimer);
    });

    

    loop {
    //    read_motion_data(&mut i2c).show();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
