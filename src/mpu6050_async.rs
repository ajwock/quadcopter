use esp_hal::Async;
use alloc::format;
use esp_println::println;
use embassy_time::{Timer, Duration};
use esp_hal::i2c::{
    master::{I2c, Operation},
};

// 7 bit address of the accelerometer
pub(crate) const ACCEL_ADDRESS: u8 = 0b1101000;

#[derive(Debug)]
pub(crate) struct Mpu6050 {
    comm: I2c<'static, Async>,
}

impl Mpu6050 {
    pub(crate) fn new(i2c: I2c<'static, Async>) -> Self {
        Self {
            comm: i2c,
        }
    }

    pub(crate) async fn write_mpu_6050_reg(&mut self, reg_address: u8, val: u8) {
        println!("Running transction");
        self.comm.write_async(ACCEL_ADDRESS, &[reg_address, val])
            .await
            .expect(format!("Failed to write val {} to register {} on mpu_6050", val, reg_address).as_str());
        println!("Finished transaction");
    }

    pub(crate) async fn burst_write_mpu_6050_regs(&mut self, start_address: u8, reg_vals: &[u8]) {
        self.comm.transaction_async(ACCEL_ADDRESS, &mut [Operation::Write(&[start_address]), Operation::Write(reg_vals)])
            .await
            .expect(format!("Failed to burst write vals {:?} to registers starting at {}", reg_vals, start_address).as_str());
    }

    // Registers have an 8-bit address
    pub(crate) async fn read_mpu_6050_reg(&mut self, reg_address: u8) -> u8 {
        let mut datum = 0;
        self.comm.write_read_async(ACCEL_ADDRESS, &[reg_address], core::slice::from_mut(&mut datum))
            .await
            .expect(format!("Failed to read register {} from mpu_6050", reg_address).as_str());
        datum
    }

    pub(crate) async fn burst_read_mpu_6050_regs(&mut self, start_address: u8, regs_out: &mut [u8]) {
        self.comm.write_read_async(ACCEL_ADDRESS, &[start_address], regs_out)
            .await
            .expect(format!("Failed to burst read from {} registers starting at {}", regs_out.len(), start_address).as_str());
    }

    pub(crate) async fn configure_mpu_6050(&mut self) {
        println!("Resetting mpu_6050");
        self.write_mpu_6050_reg(0x6B, 0x80).await; // Reset configurations
        Timer::after(Duration::from_millis(100)).await;
        self.write_mpu_6050_reg(0x6B, 0x00).await; // Wake up via power management 
        println!("mpu_6050 reset and reawoken, writing config regs");

        let config_vals = [
            3, // 25, SMPRT_DIV, Sample Rate Div 3 (400hz)
            0b00_000_010, // 26, CONFIG: FSYNC disabled, DLPF cutoff 94hz
            0b000_10_000, // 27, GYRO_CONFIG: Self test off, +-1000 degrees/sec
            0b000_01_000, // 28, ACCEL_CONFIG: Self test off, +-4g
        ];
        self.burst_write_mpu_6050_regs(25, &config_vals).await;
        println!("Successfully wrote config regs 25-28");

        println!("Disabling I2C master mode");
        self.write_mpu_6050_reg(0x6A, 0x00).await;
        println!("Enabling bypass mode");
        self.write_mpu_6050_reg(0x37, 0x02).await;
    }

    pub(crate) async fn read_motion_data(&mut self) -> MotionData {
        let mut regs_out = [0; 14];
        self.burst_read_mpu_6050_regs(59, &mut regs_out).await;
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
}



pub(crate) struct MotionData {
    acc_x: i16,
    acc_y: i16,
    acc_z: i16,
    gyr_x: i16,
    gyr_y: i16,
    gyr_z: i16,
}

impl MotionData {
    pub(crate) fn show(&self) {
        println!("Acceleration: {{ x: {}, y: {}, z: {} }}, Gyro: {{ x: {}, y: {}, z: {} }}", self.acc_x, self.acc_y, self.acc_z, self.gyr_x, self.gyr_y, self.gyr_z);
    }
}


