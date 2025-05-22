use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntfConfig10<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntfConfig10<'a, C> {
    pub fn read(&mut self) -> Result<IntfConfig10Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x25, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig10Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntfConfig10Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x25, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig10Val(val))
    }
    pub fn write(&mut self, val: IntfConfig10Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x25, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntfConfig10Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntfConfig10Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x25, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig10Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntfConfig10Val) -> IntfConfig10Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntfConfig10Val) -> IntfConfig10Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntfConfig10Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig10Val(0x0)).await
    }
}
pub struct IntfConfig10Val(pub u8);
impl IntfConfig10Val {
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
        Self(0x0)
    }
    pub fn asynctime0_dis<'a>(&'a mut self) -> FieldAsynctime0Dis<'a> {
        FieldAsynctime0Dis(self)
    }
}
pub struct FieldAsynctime0Dis<'a>(pub &'a mut IntfConfig10Val);
impl<'a> FieldAsynctime0Dis<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig10Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig10Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig10Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig10Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
