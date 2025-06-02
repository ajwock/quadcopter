#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::i2c;
use esp_hal::time::Rate;
use esp_hal::gpio::{
    Output,
    Level,
    OutputConfig,
};
use esp_println::println;


extern crate alloc;

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

    let mut led = Output::new(
        peripherals.GPIO46,
        Level::High,
        OutputConfig::default(),
    );

    let mut i2c = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default()
            .with_frequency(Rate::from_khz(50)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO47)
    .with_scl(peripherals.GPIO48);

    // TODO: Spawn some tasks
    let _ = spawner;

    Timer::after(Duration::from_millis(500)).await;

    let mut buf = [0u8];
    i2c.write_read(61, &[0x0a], &mut buf).unwrap();
    i2c.write(60, &[0xff, 0x01]).unwrap();
    println!("Device id: {:?}", buf);
    loop {
        Timer::after(Duration::from_millis(300)).await;
        led.toggle();
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
