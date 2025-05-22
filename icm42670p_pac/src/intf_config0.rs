use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntfConfig0<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> IntfConfig0<'a, D, C> {
    pub fn read(&mut self) -> Result<IntfConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x35, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntfConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x35, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig0Val(val))
    }
    pub fn write(&mut self, val: IntfConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x35, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntfConfig0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntfConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x35, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntfConfig0Val) -> IntfConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntfConfig0Val) -> IntfConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntfConfig0Val(0x30))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig0Val(0x30)).await
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
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0x30)
    }
    pub fn fifo_count_format<'a>(&'a mut self) -> FieldFifoCountFormat<'a> {
        FieldFifoCountFormat(self)
    }
    pub fn fifo_count_endian<'a>(&'a mut self) -> FieldFifoCountEndian<'a> {
        FieldFifoCountEndian(self)
    }
    pub fn sensor_data_endian<'a>(&'a mut self) -> FieldSensorDataEndian<'a> {
        FieldSensorDataEndian(self)
    }
}
pub struct FieldFifoCountFormat<'a>(pub &'a mut IntfConfig0Val);
impl<'a> FieldFifoCountFormat<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x30;
        self.0
    }
}
pub struct FieldFifoCountEndian<'a>(pub &'a mut IntfConfig0Val);
impl<'a> FieldFifoCountEndian<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x30;
        self.0
    }
}
pub struct FieldSensorDataEndian<'a>(pub &'a mut IntfConfig0Val);
impl<'a> FieldSensorDataEndian<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x30;
        self.0
    }
}
