use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl1<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl1<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x10, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x10, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl1Val(val))
    }
    pub fn write(&mut self, val: Ctrl1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x10, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x10, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl1Val) -> Ctrl1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl1Val) -> Ctrl1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl1Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl1Val(0x0)).await
    }
}
pub struct Ctrl1Val(pub u8);
impl Ctrl1Val {
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
    pub fn op_mode_xl<'a>(&'a mut self) -> FieldOpModeXl<'a> {
        FieldOpModeXl(self)
    }
    pub fn odr_xl<'a>(&'a mut self) -> FieldOdrXl<'a> {
        FieldOdrXl(self)
    }
}
pub struct FieldOpModeXl<'a>(pub &'a mut Ctrl1Val);
impl<'a> FieldOpModeXl<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl1Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl1Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= 0x0 & (!(!0 << 3) << 4);
        self.0
    }
}
pub struct FieldOdrXl<'a>(pub &'a mut Ctrl1Val);
impl<'a> FieldOdrXl<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl1Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl1Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x0 & (!(!0 << 4) << 0);
        self.0
    }
}
