
use crate::mpu6050_async::{Mpu6050, MotionData};
use embassy_executor::Spawner;
use embassy_sync::{
    channel::{Channel, Sender},
    blocking_mutex::raw::NoopRawMutex,
};
use embassy_time::{Duration, Ticker};
use esp_println::println;
use esp_hal::{
    dma::{DmaRxBuf, DmaTxBuf},
    dma_buffers,
    i2c::master::I2c,
    time::Rate,
    timer::{
        timg::TimerGroup,
        PeriodicTimer,
    },
    Async,
    handler,
};
use core::cell::RefCell;
use static_cell::StaticCell;
use no_std_strings::str64;

type ChannelMutex = NoopRawMutex;

type MotionResult = Result<(Mpu6050<I2c<'static, Async>>, MotionData), (Mpu6050<I2c<'static, Async>>, str64)>;

#[embassy_executor::task]
async fn read_motion_data(mut mpu: Mpu6050<I2c<'static, Async>>, tx: Sender<'static, ChannelMutex, MotionResult, 1>) {
    let data = mpu.read_motion_data().await;
    tx.send(Ok((mpu, data))).await;
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let i2c = I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default()
            .with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO0)
    .with_sda(peripherals.GPIO1)
    .into_async();
    let mut mpu = Mpu6050::new(i2c);
    println!("Configuring mpu 6050...");
    mpu.configure_mpu_6050().await;
    println!("Configured mpu 6050 successfully");

    static MOTION_DATA_CHANNEL: StaticCell<Channel<ChannelMutex, MotionResult, 1>> = StaticCell::new();
    let motion_data_channel = MOTION_DATA_CHANNEL.init(Channel::new());
    let receiver = motion_data_channel.receiver();
    let mut ticker = Ticker::every(Duration::from_millis(100));
    println!("We're async for real now, hasse");
    {
        spawner.spawn(read_motion_data(mpu, motion_data_channel.sender())).unwrap();
        // Do something else

        let (mpu, motion_data) = receiver.receive().await
            .unwrap();
        motion_data.show();
        ticker.next().await;
    }
}
