use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct OffsetUser7<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> OffsetUser7<'a, D, C> {
    pub fn read(&mut self) -> Result<OffsetUser7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x55, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(OffsetUser7Val(val))
    }
    pub async fn read_async(&mut self) -> Result<OffsetUser7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x55, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(OffsetUser7Val(val))
    }
    pub fn write(&mut self, val: OffsetUser7Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x55, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(OffsetUser7Val(raw_val))
    }
    pub async fn write_async(&mut self, val: OffsetUser7Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x55, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(OffsetUser7Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(OffsetUser7Val) -> OffsetUser7Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(OffsetUser7Val) -> OffsetUser7Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(OffsetUser7Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(OffsetUser7Val(0x0)).await
    }
}
pub struct OffsetUser7Val(pub u8);
impl OffsetUser7Val {
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
    pub fn accel_z_offuser_11_8<'a>(&'a mut self) -> FieldAccelZOffuser118<'a> {
        FieldAccelZOffuser118(self)
    }
    pub fn accel_y_offuser_11_8<'a>(&'a mut self) -> FieldAccelYOffuser118<'a> {
        FieldAccelYOffuser118(self)
    }
}
pub struct FieldAccelZOffuser118<'a>(pub &'a mut OffsetUser7Val);
impl<'a> FieldAccelZOffuser118<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut OffsetUser7Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut OffsetUser7Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= 0x0 & (!(!0 << 4) << 4);
        self.0
    }
}
pub struct FieldAccelYOffuser118<'a>(pub &'a mut OffsetUser7Val);
impl<'a> FieldAccelYOffuser118<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut OffsetUser7Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut OffsetUser7Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x0 & (!(!0 << 4) << 0);
        self.0
    }
}
