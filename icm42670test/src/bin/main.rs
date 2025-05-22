#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Ticker};
use esp_hal::clock::CpuClock;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::i2c;
use esp_println as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

use icm42670::{
    Icm42670,
    AccelConfig,
    GyroConfig,
};
use regcomms::i2c::I2cCommsAsync;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;
    let i2c = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default()
            .with_frequency(Rate::from_khz(1000)))
    .unwrap()
    .with_sda(peripherals.GPIO10)
    .with_sda(peripherals.GPIO4)
    .into_async();
    let i2c_async = I2cCommsAsync::new(i2c)
        .with_address(0b1101000);
    let mut icm = Icm42670::new(i2c_async, Delay);
    info!("Power on...");
    icm.poweron_idle().await.unwrap();
    info!("Verifying identity...");
    icm.verify_identity().await.unwrap();
    info!("Configuring...");
    icm.configure(Default::default()).await.unwrap();
    info!("Configured successfully");

    let mut ticker = Ticker::every(Duration::from_millis(100));
    loop {
        info!("Hello world!");
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
