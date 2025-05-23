use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl2<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl2<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x11, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x11, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl2Val(val))
    }
    pub fn write(&mut self, val: Ctrl2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x11, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl2Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x11, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl2Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl2Val) -> Ctrl2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl2Val) -> Ctrl2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl2Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl2Val(0x0)).await
    }
}
pub struct Ctrl2Val(pub u8);
impl Ctrl2Val {
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
    pub fn op_mode_g<'a>(&'a mut self) -> FieldOpModeG<'a> {
        FieldOpModeG(self)
    }
    pub fn odr_g<'a>(&'a mut self) -> FieldOdrG<'a> {
        FieldOdrG(self)
    }
}
pub struct FieldOpModeG<'a>(pub &'a mut Ctrl2Val);
impl<'a> FieldOpModeG<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl2Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl2Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= 0x0 & (!(!0 << 3) << 4);
        self.0
    }
}
pub struct FieldOdrG<'a>(pub &'a mut Ctrl2Val);
impl<'a> FieldOdrG<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl2Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl2Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x0 & (!(!0 << 4) << 0);
        self.0
    }
}
