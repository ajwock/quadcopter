use crate::imu_common::{Imu, ImuMsg, ImuError};
use icm42670::Icm42670;
use embedded_hal_async::delay::DelayNs;
use regcomms::RegComms;

#[derive(Copy, Clone, Debug)]
pub struct FifoPacket {
    pub accel_data: Option<[i16; 3]>,
    pub gyro_data:  Option<[i16; 3]>,
    pub _temp_data:  u16,
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
/*
    fn has_odr_timestamp(&self) -> bool {
        ((self.0 >> 2) & 0b11) == 0b10
    }*/

    fn has_timestamp(&self) -> bool {
        (self.0 & (1 << 3)) != 0
    }
/*
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
    }*/
}

impl<D: DelayNs, C: RegComms<1, u8>> Imu for Icm42670<D, C> {
    async fn get_motion_data_msg(&mut self) -> Result<ImuMsg, ImuError> {
        let mut buf = [0u8; 16];
        let portion = &mut buf;
        let _ = self.p.fifo_data().data_port_read_async(portion).await;
        let header_data = portion[0];
        // Fifo empty
        if header_data == 0xFF {
            return Err(ImuError::not_ready())
        }
        let header = FifoPacketHeader(header_data);
        if !header.has_data() {
            return Err(ImuError::not_ready())
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
        let _temp_data = if header.has_20bit_ext() {
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
        FifoPacket { accel_data, gyro_data, _temp_data, timestamp }.try_into()
    }
}
