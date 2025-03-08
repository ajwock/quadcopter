#![no_std]
#![no_main]

mod mpu6050;

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

use core::cell::RefCell;
use critical_section::Mutex;
use esp_hal::handler;

extern crate alloc;


static MPU_6050: Mutex<RefCell<Option<Mpu6050<'static, Blocking>>>> = Mutex::new(RefCell::new(None));
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
    let timg1 = TimerGroup::new(peripherals.TIMG1);
    let mut ptimer = PeriodicTimer::new(timg1.timer0);
    ptimer.set_interrupt_handler(timed_interrupt_handler);
    ptimer.enable_interrupt(true);
    ptimer.start(Duration::from_millis(100))
        .expect("Failed to start periodic timer1");
    critical_section::with(|cs| {
        MPU_6050.borrow_ref_mut(cs).replace(mpu);
        TIMER.borrow_ref_mut(cs).replace(ptimer);
    });

    loop {
    //    read_motion_data(&mut i2c).show();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
