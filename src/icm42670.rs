use esp_hal::i2c::master::{I2c, Operation};
use esp_hal::Async;
use alloc::format;
use esp_println::println;
use smallvec::SmallVec;
use crate::motion_data::MotionData;
use crate::imu_common::Imu;

// 7 bit address of the accelerometer
pub(crate) const ACCEL_ADDRESS: u8 = 0b1101000;

pub(crate) struct Icm42670<'a> {
    comm: I2c<'a, Async>,
    pub(crate) calibration_offsets: MotionData,
    pub(crate) prev_motion_data: MotionData,
}

impl<'a> Icm42670<'a> {
    pub(crate) fn new(i2c: I2c<'a, Async>) -> Self {
        Self {
            comm: i2c,
            calibration_offsets: MotionData::zero(),
            prev_motion_data: MotionData::zero(),
        }
    }

    async fn write_reg(&mut self, reg_address: u8, val: u8) {
        self.comm.write_async(ACCEL_ADDRESS, &[reg_address, val])
            .await
            .expect(format!("Failed to write val {} to register {}", val, reg_address).as_str());
    }

    async fn burst_write_regs(&mut self, start_address: u8, reg_vals: &[u8]) {
        let mut to_write = SmallVec::<[u8; 32]>::new();
        to_write.push(start_address);
        to_write.extend_from_slice(reg_vals);
        self.comm.write_async(ACCEL_ADDRESS, to_write.as_slice())
            .await
            .expect(format!("Failed to burst write vals {:?} to registers starting at {}", reg_vals, start_address).as_str());
    }

    // Registers have an 8-bit address
    async fn read_reg(&mut self, reg_address: u8) -> u8 {
        let mut datum = 0;
        self.comm.write_read_async(ACCEL_ADDRESS, &[reg_address], core::slice::from_mut(&mut datum))
            .await
            .expect(format!("Failed to read register {}", reg_address).as_str());
        datum
    }

    async fn burst_read_regs(&mut self, start_address: u8, regs_out: &mut [u8]) -> Result<(), ()> {
        self.comm.write_read_async(ACCEL_ADDRESS, &[start_address], regs_out)
            .await
            .map_err(|_| {
                println!("Failed to burst read from {} registers starting at {}", regs_out.len(), start_address);
            ()
        })
    }

    pub async fn configure(&mut self) {
        println!("Power on ICM42670");
        self.write_reg(0x1f, 0x0f).await;
        println!("ICM42680 powered on, verifying identity");
        let id = self.read_reg(0x75).await;
        println!("Got id: 0x{:x}", id);
        if id != 0x67 {
            panic!("Device not identified as icm42760, expected 0x67 but got 0x{:x}", id);
        }
        println!("Configuring accelerometer and gyro");
        let configuration_data = &[
            0b0100_0110, // Gyro ODR selection: 400hz
            0b0100_0111, // Accel ODR selection: 400hz,
            0b0100_0000, // Temp DLPF: 16Hz,
            0b0000_0010, // Gyro DLPF: 121Hz,
            0b0000_0010, // Accel DLPF: 121Hz,
        ];
        self.burst_write_regs(0x20, configuration_data).await;
        println!("ICM42680 configured");
    }

    pub async fn read_motion_data(&mut self) -> MotionData {
        let mut outbuf = [0; 12];
        if let Err(_) = self.burst_read_regs(0x0b, &mut outbuf).await {
            return self.prev_motion_data
        }
        let out = MotionData {
            acc_x: i16::from_be_bytes([outbuf[0], outbuf[1]]),
            acc_y: i16::from_be_bytes([outbuf[2], outbuf[3]]),
            acc_z: i16::from_be_bytes([outbuf[4], outbuf[5]]),
            gyr_x: i16::from_be_bytes([outbuf[6], outbuf[7]]),
            gyr_y: i16::from_be_bytes([outbuf[8], outbuf[9]]),
            gyr_z: i16::from_be_bytes([outbuf[10], outbuf[11]]),
        };
        self.prev_motion_data = out;
        out
    }
}

impl Imu for Icm42670<'_> {
    async fn read_motion_data_raw(&mut self) -> MotionData {
        self.read_motion_data().await
    }
}
