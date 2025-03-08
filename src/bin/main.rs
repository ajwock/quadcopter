#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::timer::PeriodicTimer;
use esp_hal::{
    DriverMode,
    Blocking,
};
use log::info;
use esp_hal::i2c;
use esp_hal::i2c::master::Operation;
use alloc::format;
use esp_println::{print, println};

use core::cell::RefCell;
use critical_section::Mutex;
use esp_hal::{
    delay::Delay,
    handler
};

extern crate alloc;
// 7 bit address of the accelerometer
pub const ACCEL_ADDRESS: u8 = 0b1101000;


fn write_mpu_6050_reg<D: DriverMode>(comm: &mut i2c::master::I2c<'_, D>, reg_address: u8, val: u8) {
    // send start
    // send accel_address + write bit, get ack
    // send register address, get ack
    // send data, get ack
    // send stop
    // ... Actually we could do burst read or burst write, with auto incremented reg address
    comm.write(ACCEL_ADDRESS, &[reg_address, val])
        .expect(format!("Failed to write val {} to register {} on mpu_6050", val, reg_address).as_str());
}

fn burst_write_mpu_6050_regs<D: DriverMode>(comm: &mut i2c::master::I2c<'_, D>, start_address: u8, reg_vals: &[u8]) {
    comm.transaction(ACCEL_ADDRESS, &mut [Operation::Write(&[start_address]), Operation::Write(reg_vals)])
        .expect(format!("Failed to burst write vals {:?} to registers starting at {}", reg_vals, start_address).as_str());
}

// Registers have an 8-bit address
// TODO: make fallible, do error recovery?
fn read_mpu_6050_reg<D: DriverMode>(comm: &mut i2c::master::I2c<'_, D>, reg_address: u8) -> u8 {
    // i2c start
    // send accel_address + write bit, get ack
    // send register address, get ack
    // i2c start
    // send accel_address + read bit, get ack
    // get data, send ack
    // send nack
    // send stop
    let mut datum = 0;
    comm.write_read(ACCEL_ADDRESS, &[reg_address], core::slice::from_mut(&mut datum))
        .expect(format!("Failed to read register {} from mpu_6050", reg_address).as_str());
    datum
}

fn burst_read_mpu_6050_regs<D: DriverMode>(comm: &mut i2c::master::I2c<'_, D>, start_address: u8, regs_out: &mut [u8]) {
    comm.write_read(ACCEL_ADDRESS, &[start_address], regs_out)
        .expect(format!("Failed to burst read from {} registers starting at {}", regs_out.len(), start_address).as_str());
}

fn configure_mpu_6050<D: DriverMode>(comm: &mut i2c::master::I2c<'_, D>) {
    println!("Resetting mpu_6050");
    write_mpu_6050_reg(comm, 0x6B, 0x80); // Reset configurations
    esp_hal::delay::Delay::new().delay_millis(100);
    write_mpu_6050_reg(comm, 0x6B, 0x00); // Wake up via power management 
    println!("mpu_6050 reset and reawoken, writing config regs");

    let config_vals = [
        3, // 25, SMPRT_DIV, Sample Rate Div 3 (400hz)
        0b00_000_010, // 26, CONFIG: FSYNC disabled, DLPF cutoff 94hz
        0b000_10_000, // 27, GYRO_CONFIG: Self test off, +-1000 degrees/sec
        0b000_01_000, // 28, ACCEL_CONFIG: Self test off, +-4g
    ];
    burst_write_mpu_6050_regs(comm, 25, &config_vals);
    println!("Successfully wrote config regs 25-28");

    println!("Disabling I2C master mode");
    write_mpu_6050_reg(comm, 0x6A, 0x00);
    println!("Enabling bypass mode");
    write_mpu_6050_reg(comm, 0x37, 0x02);
}

struct MotionData {
    acc_x: i16,
    acc_y: i16,
    acc_z: i16,
    gyr_x: i16,
    gyr_y: i16,
    gyr_z: i16,
}

impl MotionData {
    fn show(&self) {
        println!("Acceleration: {{ x: {}, y: {}, z: {} }}, Gyro: {{ x: {}, y: {}, z: {} }}", self.acc_x, self.acc_y, self.acc_z, self.gyr_x, self.gyr_y, self.gyr_z);
    }
}

fn read_motion_data<D: DriverMode>(comm: &mut i2c::master::I2c<'_, D>) -> MotionData {
    let mut regs_out = [0; 14];
    burst_read_mpu_6050_regs(comm, 59, &mut regs_out);
    let acc_x = i16::from_be_bytes([regs_out[0], regs_out[1]]);
    let acc_y = i16::from_be_bytes([regs_out[2], regs_out[3]]);
    let acc_z = i16::from_be_bytes([regs_out[4], regs_out[5]]);
    // Skip the temperature regs
    let gyr_x = i16::from_be_bytes([regs_out[8], regs_out[9]]);
    let gyr_y = i16::from_be_bytes([regs_out[10], regs_out[11]]);
    let gyr_z = i16::from_be_bytes([regs_out[12], regs_out[13]]);
    MotionData {
        acc_x,
        acc_y,
        acc_z,
        gyr_x,
        gyr_y,
        gyr_z,
    }
}

static MPU_COMM: Mutex<RefCell<Option<i2c::master::I2c<'static, Blocking>>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<PeriodicTimer<'static, Blocking>>>> = Mutex::new(RefCell::new(None));

#[handler]
fn timed_interrupt_handler() {
    critical_section::with(|cs| {
        let mut comm_borrow = MPU_COMM.borrow_ref_mut(cs);
        let comm_ref = comm_borrow
            .as_mut()
            .unwrap();
        let motion_data = read_motion_data(comm_ref);
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

    let mut i2c = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default()
            .with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO0)
    .with_scl(peripherals.GPIO1);

    println!("Configuring mpu 6050");
    configure_mpu_6050(&mut i2c);
    println!("mpu 6050 configured");
    let timg1 = TimerGroup::new(peripherals.TIMG1);
    let mut ptimer = PeriodicTimer::new(timg1.timer0);
    ptimer.set_interrupt_handler(timed_interrupt_handler);
    ptimer.enable_interrupt(true);
    ptimer.start(Duration::from_millis(100))
        .expect("Failed to start periodic timer1");
    critical_section::with(|cs| {
        MPU_COMM.borrow_ref_mut(cs).replace(i2c);
        TIMER.borrow_ref_mut(cs).replace(ptimer);
    });

    loop {
    //    read_motion_data(&mut i2c).show();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
