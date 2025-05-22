use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct AccelConfig0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> AccelConfig0<'a, C> {
    pub fn read(&mut self) -> Result<AccelConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x21, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<AccelConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x21, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelConfig0Val(val))
    }
    pub fn write(&mut self, val: AccelConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x21, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(AccelConfig0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: AccelConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x21, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(AccelConfig0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(AccelConfig0Val) -> AccelConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(AccelConfig0Val) -> AccelConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(AccelConfig0Val(0x6))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(AccelConfig0Val(0x6)).await
    }
}
pub struct AccelConfig0Val(pub u8);
impl AccelConfig0Val {
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
        Self(0x6)
    }
    pub fn accel_ui_fs_sel<'a>(&'a mut self) -> FieldAccelUiFsSel<'a> {
        FieldAccelUiFsSel(self)
    }
    pub fn accel_odr<'a>(&'a mut self) -> FieldAccelOdr<'a> {
        FieldAccelOdr(self)
    }
}
pub struct FieldAccelUiFsSel<'a>(pub &'a mut AccelConfig0Val);
impl<'a> FieldAccelUiFsSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut AccelConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 5;
        self.0
    }
    pub fn reset(self) -> &'a mut AccelConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= 0x6 & (!(!0 << 2) << 5);
        self.0
    }
}
pub struct FieldAccelOdr<'a>(pub &'a mut AccelConfig0Val);
impl<'a> FieldAccelOdr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut AccelConfig0Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut AccelConfig0Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x6 & (!(!0 << 4) << 0);
        self.0
    }
}
