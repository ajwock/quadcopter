use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntfConfig0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntfConfig0<'a, C> {
    pub fn read(&mut self) -> Result<IntfConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x35, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntfConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x35, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig0Val(val))
    }
    pub fn write(&mut self, val: IntfConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x35, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: IntfConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x35, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct IntfConfig0Val(pub u8);
impl IntfConfig0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn fifo_count_format<'a>(&'a mut self) -> FifoCountFormat<'a> {
        FifoCountFormat(self)
    }
    pub fn fifo_count_endian<'a>(&'a mut self) -> FifoCountEndian<'a> {
        FifoCountEndian(self)
    }
    pub fn sensor_data_endian<'a>(&'a mut self) -> SensorDataEndian<'a> {
        SensorDataEndian(self)
    }
}
pub struct FifoCountFormat<'a>(pub &'a mut IntfConfig0Val);
impl<'a> FifoCountFormat<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= !(!(val as u8) << 6);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(false)
    }
}
pub struct FifoCountEndian<'a>(pub &'a mut IntfConfig0Val);
impl<'a> FifoCountEndian<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= !(!(val as u8) << 5);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(false)
    }
}
pub struct SensorDataEndian<'a>(pub &'a mut IntfConfig0Val);
impl<'a> SensorDataEndian<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= !(!(val as u8) << 4);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(false)
    }
}
