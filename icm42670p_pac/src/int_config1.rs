use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntConfig1<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> IntConfig1<'a, D, C> {
    pub fn read(&mut self) -> Result<IntConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x5, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x5, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntConfig1Val(val))
    }
    pub fn write(&mut self, val: IntConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x5, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntConfig1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x5, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntConfig1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntConfig1Val) -> IntConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntConfig1Val) -> IntConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntConfig1Val(0x10))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntConfig1Val(0x10)).await
    }
}
pub struct IntConfig1Val(pub u8);
impl IntConfig1Val {
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
        Self(0x10)
    }
    pub fn int_tpulse_duration<'a>(&'a mut self) -> FieldIntTpulseDuration<'a> {
        FieldIntTpulseDuration(self)
    }
    pub fn int_async_reset<'a>(&'a mut self) -> FieldIntAsyncReset<'a> {
        FieldIntAsyncReset(self)
    }
}
pub struct FieldIntTpulseDuration<'a>(pub &'a mut IntConfig1Val);
impl<'a> FieldIntTpulseDuration<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfig1Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfig1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntConfig1Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x10;
        self.0
    }
}
pub struct FieldIntAsyncReset<'a>(pub &'a mut IntConfig1Val);
impl<'a> FieldIntAsyncReset<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfig1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfig1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntConfig1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x10;
        self.0
    }
}
