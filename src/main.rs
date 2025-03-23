#![no_std]
#![no_main]

mod icm42670;
mod imu_common;
mod motor_drive;
mod motion_data;
mod utils;

use esp_println::println;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Ticker};
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

extern crate alloc;

static IMU_START_READ: Signal<CriticalSectionRawMutex, ()> = Signal::new();
static IMU_READ_DONE: Signal<CriticalSectionRawMutex, MotionData> = Signal::new();

#[embassy_executor::task]
async fn imu_read_task(mut imu: Icm42670<'static>) {
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

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

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

    // TODO: Spawn some tasks
    spawner
        .spawn(imu_read_task(imu)).unwrap();
    let _ = spawner;
    let mut ticker = Ticker::every(Duration::from_millis(100));
    let mut prev_motiondata = MotionData::zero();
    loop {
        IMU_START_READ.signal(());
        prev_motiondata.show();
        let motion_data = IMU_READ_DONE.wait().await;
        prev_motiondata = motion_data;
        ticker.next().await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
