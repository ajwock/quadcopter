use esp_hal::i2c::master::{I2c, Operation};
use esp_hal::Async;
use alloc::format;
use esp_println::println;
use crate::debug_println;
use smallvec::SmallVec;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use crate::motion_data::MotionData;
use core::convert::TryFrom;
use crate::imu_common::{
    Imu,
    ImuMsg,
    ImuError,
};
use esp_hal::i2c;

// 7 bit address of the accelerometer
pub const ACCEL_ADDRESS: u8 = 0b1101000;

pub struct Icm42670<'a> {
    comm: I2c<'a, Async>,
    pub calibration_offsets: MotionData,
    pub prev_motion_data: MotionData,
}

#[derive(Copy, Clone, Debug)]
pub enum BlkSel {
    MREG1,
    MREG2,
    MREG3,
}

impl BlkSel {
    fn block_sel_val(self) -> u8 {
        match self {
            Self::MREG1 => 0,
            Self::MREG2 => 0x28,
            Self::MREG3 => 0x50,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AccelRange {
    G16,
    G8,
    G4,
    G2,
}

impl Default for AccelRange {
    fn default() -> Self {
        Self::G16
    }
}

impl AccelRange {
    fn to_bits(self) -> u8{
        match self {
            Self::G16 => 0b00,
            Self::G8  => 0b01,
            Self::G4  => 0b10,
            Self::G2  => 0b11,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GyroRange {
    DPS2000,
    DPS1000,
    DPS500,
    DPS250,
}

impl Default for GyroRange {
    fn default() -> Self {
        Self::DPS2000
    }
}

impl GyroRange {
    fn to_bits(self) -> u8 {
        match self {
            Self::DPS2000 => 0b00,
            Self::DPS1000 => 0b01,
            Self::DPS500  => 0b10,
            Self::DPS250  => 0b11,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DLPF {
    Bypassed,
    Hz180,
    Hz121,
    Hz73,
    Hz53,
    Hz34,
    Hz25,
    Hz16,
}

impl Default for DLPF {
    fn default() -> Self {
        Self::Bypassed
    }
}

impl DLPF {
    fn to_bits(self) -> u8 {
        match self {
            Self::Bypassed => 0b000,
            Self::Hz180 =>    0b001,
            Self::Hz121 =>    0b010,
            Self::Hz73 =>     0b011,
            Self::Hz53 =>     0b100,
            Self::Hz34 =>     0b101,
            Self::Hz25 =>     0b110,
            Self::Hz16 =>     0b111,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ODR {
    Hz1600,
    Hz800,
    Hz400,
    Hz200,
    Hz100,
    Hz50,
    Hz25,
    Hz12_5,
}

impl Default for ODR {
    fn default() -> Self {
        Self::Hz1600
    }
}

impl ODR {
    fn to_bits(self) -> u8 {
        match self {
            Self::Hz1600 => 0b0101,
            Self::Hz800 =>  0b0110,
            Self::Hz400 =>  0b0111,
            Self::Hz200 =>  0b1000,
            Self::Hz100 =>  0b1001,
            Self::Hz50  =>  0b1010,
            Self::Hz25  =>  0b1011,
            Self::Hz12_5 => 0b1100,
       }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct AccelConfig {
    pub accel_range: AccelRange,
    pub accel_odr: ODR,
    pub accel_dlpf: DLPF,
}

impl AccelConfig {
    fn accel_config0(&self) -> u8 {
        self.accel_range.to_bits() << 5 |
            self.accel_odr.to_bits()
    }

    fn accel_config1(&self) -> u8 {
        // Not using avg for now
        self.accel_dlpf.to_bits()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FifoPacket {
    pub accel_data: Option<[i16; 3]>,
    pub gyro_data:  Option<[i16; 3]>,
    pub temp_data:  u16,
    pub timestamp:  Option<u16>,
}

impl TryFrom<FifoPacket> for ImuMsg {
    type Error = ImuError;
    fn try_from(pkt: FifoPacket) -> Result<Self, Self::Error> {
        let (Some(accel_data), Some(gyro_data), Some(timestamp)) = (
            pkt.accel_data,
            pkt.gyro_data,
            pkt.timestamp
        ) else {
            return Err(ImuError::missing_info())
        };
        Ok(ImuMsg::new(accel_data, gyro_data, timestamp))
    }
}

struct FifoPacketHeader(u8);

impl FifoPacketHeader {
    fn has_data(&self) -> bool {
        (self.0 & (1 << 7)) == 0
    }

    fn has_accel(&self) -> bool {
        (self.0 & (1 << 6)) != 0
    }

    fn has_gyro(&self) -> bool {
        (self.0 & (1 << 5)) != 0
    }

    fn has_20bit_ext(&self) -> bool {
        (self.0 & (1 << 4)) != 0
    }

    fn has_odr_timestamp(&self) -> bool {
        ((self.0 >> 2) & 0b11) == 0b10
    }

    fn has_timestamp(&self) -> bool {
        (self.0 & (1 << 3)) != 0
    }

    fn accel_new(&self) -> bool {
        (self.0 & (1 << 1)) != 0
    }

    fn gyro_new(&self) -> bool {
        (self.0 & (1 << 0)) != 0
    }
    fn packet_size(&self) -> usize {
        let mut size = 2; // Header + temp_data
        size += self.has_accel().then_some(6).unwrap_or(0);
        size += self.has_gyro().then_some(6).unwrap_or(0);
        size += self.has_20bit_ext().then_some(4).unwrap_or(0);
        size += self.has_timestamp().then_some(2).unwrap_or(0);
        size
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct GyroConfig {
    pub gyro_range: GyroRange,
    pub gyro_odr: ODR,
    pub gyro_dlpf: DLPF,
}

impl GyroConfig {
    fn gyro_config0(&self) -> u8 {
        self.gyro_range.to_bits() << 5 |
            self.gyro_odr.to_bits()
    }

    fn gyro_config1(&self) -> u8 {
        self.gyro_dlpf.to_bits()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FifoMode {
    Stream,
    StopOnFull,
}

impl Default for FifoMode {
    fn default() -> Self {
        Self::Stream
    }
}

impl FifoMode {
    fn to_bits(self) -> u8 {
        match self {
            Self::Stream => 0b0,
            Self::StopOnFull => 0b1,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FifoConfig {
    pub watermark: Option<u16>,
    pub mode: FifoMode,
}

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub accel_config: Option<AccelConfig>,
    pub gyro_config:  Option<GyroConfig>,
    pub fifo_config:  Option<FifoConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            accel_config: Some(Default::default()),
            gyro_config:  Some(Default::default()),
            fifo_config:  None,
        }
    }
}

impl<'a> Icm42670<'a> {
    pub fn new(i2c: I2c<'a, Async>) -> Self {
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
        self.comm.write_read(ACCEL_ADDRESS, &[start_address], regs_out)
            .map_err(|e| {
                panic!("Failed to burst read from {} registers starting at {}: {:?}", regs_out.len(), start_address, e);
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
            0b0000_1010, // +-2000 deg/s, Gyro ODR selection: 50hz
            0b0100_1010, // +-8g, Accel ODR selection: 50hz,
            0b0100_0000, // Temp DLPF: 16Hz,
            0b0000_0010, // Gyro DLPF: 121Hz,
            0b0000_0010, // Accel DLPF: 121Hz,
        ];
        self.burst_write_regs(0x20, configuration_data).await;
        println!("ICM42680 configured");
    }

    pub async fn write_block_reg(&mut self, blk: BlkSel, address: u8, value: u8) {
        let block_sel = blk.block_sel_val();
        self.burst_write_regs(0x79, &[block_sel, address]).await;
        // Note, per section 14 of the datasheet we must wait 10us before write,
        // 10us after write, then reset the blk_sel_w is 0 afterward
        let mut d = embassy_time::Delay;
        d.delay_us(10).await;
        self.write_reg(0x7b, value).await;
        d.delay_us(10).await;
        self.write_reg(0x7a, 0).await;
    }

    pub async fn read_block_reg(&mut self, blk: BlkSel, address: u8) -> u8 {
        let block_sel = blk.block_sel_val();
        self.burst_write_regs(0x7C, &[block_sel, address]).await;
        let mut d = embassy_time::Delay;
        d.delay_us(10).await;
        let read_val = self.read_reg(0x7e).await;
        d.delay_us(10).await;
        self.write_reg(0x7c, 0).await;
        read_val
    }

    pub async fn fifo_configure(&mut self, conf: Config) {
        let Some(fifo_config) = conf.fifo_config else {
            return
        };
        // Disable anything first?
        let mut conf5_flags = 0;
        if let Some(wm) = fifo_config.watermark {
            let bytes = wm.to_be_bytes();
            self.burst_write_regs(0x29, &[bytes[1], bytes[0]]).await;
        }
        if conf.accel_config.is_some() {
            conf5_flags |= 1 << 0;
        }
        if conf.gyro_config.is_some() {
            conf5_flags |= 1 << 1;
        }
        // Enable timestamp.  Possibly essential to avoid bad packets?
        self.write_block_reg(BlkSel::MREG1, 0x0, 0b01).await;
        self.write_block_reg(BlkSel::MREG1, 0x1, conf5_flags).await;
        // Not using ALP/WUOSC so disable wakeup
        self.write_block_reg(BlkSel::MREG1, 0x2, 0x1).await;
        // Disable APEX for bigger FIFO
        self.write_block_reg(BlkSel::MREG1, 0x6, 1 << 6).await;
        let readback = self.read_block_reg(BlkSel::MREG1, 0x1).await;
        if readback != conf5_flags {
            println!("Fifo config failed- sent conf val 0x{:x} but read back 0x{:x}", conf5_flags, readback);
        }
        println!("Got matching configs: 0x{:x} and 0x{:x}", conf5_flags, readback);
        println!("Enabling fifo");
        let mut conf1_flags = 0;
        conf1_flags |= fifo_config.mode.to_bits() << 1;
        self.write_reg(0x28, conf1_flags).await;
    }

    pub async fn configure2(&mut self, conf: Config) {
        println!("Idle on ICM42670");
        self.write_reg(0x1f, 0b00010000).await;
        let mut d = embassy_time::Delay;
        d.delay_us(200).await;
        println!("ICM42680 powered on, verifying identity");
        let id = self.read_reg(0x75).await;
        println!("Got id: 0x{:x}", id);
        if id != 0x67 {
            panic!("Device not identified as icm42760, expected 0x67 but got 0x{:x}", id);
        }
        let acc_cnf = conf.accel_config.unwrap_or_default();
        let gyro_cnf = conf.gyro_config.unwrap_or_default();
        let configuration_data = &[
            gyro_cnf.gyro_config0(),
            acc_cnf.accel_config0(),
            0b0100_0000, // Temp with DLPF: 16Hz
            gyro_cnf.gyro_config1(),
            acc_cnf.accel_config1()
        ];
        self.burst_write_regs(0x20, configuration_data).await;
        self.fifo_configure(conf).await;
    }

    pub async fn full_enable(&mut self) {
        self.write_reg(0x1f, 0x0f).await;
        let mut d = embassy_time::Delay;
        d.delay_us(200).await;
    }

    pub async fn flush_fifo(&mut self) {
        self.write_reg(0x02, 1 << 2).await;
        embassy_time::Delay.delay_ns(1500).await;
        let flushed = (self.read_reg(0x02).await & 0b100) == 0;
        if !flushed {
            println!("Note, failed to flush fifo");
        }
    }

    pub async fn read_fifo_packet(&mut self) -> Result<Option<FifoPacket>, i2c::master::Error> {
        let mut buf = [0; 16];
        let portion = &mut buf[0..16];
        let _ = self.burst_read_regs(0x3f, portion).await.map_err(|e|
            panic!("Failed to burst read: {:?}", e));
        debug_println!("Fifo packet: {:?}", portion);
        let header_data = portion[0];
        debug_println!("Header: {header_data}");
        // Fifo empty
        if header_data == 0xFF {
            return Ok(None)
        }
        let header = FifoPacketHeader(header_data);
        if !header.has_data() {
            return Ok(None)
        }
        let (&mut h2, mut portion) = portion.split_first_mut().unwrap();
        assert!(h2 == header_data, "FIFO mode set incorrectly");
        let accel_data = if header.has_accel() {
            let (chunk, remainder): (&mut [u8; 6], _)  = portion.split_first_chunk_mut().unwrap();
            portion = remainder;
            let acc_x = i16::from_be_bytes([chunk[0], chunk[1]]);
            let acc_y = i16::from_be_bytes([chunk[2], chunk[3]]);
            let acc_z = i16::from_be_bytes([chunk[4], chunk[5]]);
            Some([acc_x, acc_y, acc_z])
        } else {
            None
        };
        let gyro_data = if header.has_gyro() {
            let (chunk, remainder): (&mut [u8; 6], _)  = portion.split_first_chunk_mut().unwrap();
            portion = remainder;
            let gyr_x = i16::from_be_bytes([chunk[0], chunk[1]]);
            let gyr_y = i16::from_be_bytes([chunk[2], chunk[3]]);
            let gyr_z = i16::from_be_bytes([chunk[4], chunk[5]]);
            Some([gyr_x, gyr_y, gyr_z])
        } else {
            None
        };
        let (&mut temp_hbits, mut portion) = portion.split_first_mut().unwrap();
        let temp_data = if header.has_20bit_ext() {
            let (&mut temp_lbits, remainder) = portion.split_first_mut().unwrap();
            portion = remainder;
            u16::from_be_bytes([temp_hbits, temp_lbits])
        } else {
            temp_hbits as u16
        };
        let timestamp = if header.has_timestamp() {
            let (&mut chunk, _remainder): (&mut [u8; 2], _) = portion.split_first_chunk_mut().unwrap();
            Some(u16::from_be_bytes(chunk))
        } else {
            None
        };
        Ok(Some(FifoPacket { accel_data, gyro_data, temp_data, timestamp }))
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

    async fn get_motion_data_msg(&mut self) -> Result<ImuMsg, ImuError> {
        let fifo_packet = match self.read_fifo_packet().await {
            Ok(Some(pkt)) => Ok(pkt),
            Ok(None) => Err(ImuError::not_ready()),
            Err(_) => Err(ImuError::comms_error()),
        }?;
        fifo_packet.try_into()
    }

    async fn flush_msgs(&mut self) {
        self.flush_fifo().await
    }
}
