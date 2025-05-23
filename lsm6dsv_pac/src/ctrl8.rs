use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl8<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl8<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl8Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x17, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl8Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl8Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x17, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl8Val(val))
    }
    pub fn write(&mut self, val: Ctrl8Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x17, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl8Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl8Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x17, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl8Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl8Val) -> Ctrl8Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl8Val) -> Ctrl8Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl8Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl8Val(0x0)).await
    }
}
pub struct Ctrl8Val(pub u8);
impl Ctrl8Val {
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
    pub fn hp_lpf2_xl_bw<'a>(&'a mut self) -> FieldHpLpf2XlBw<'a> {
        FieldHpLpf2XlBw(self)
    }
    pub fn fs_xl<'a>(&'a mut self) -> FieldFsXl<'a> {
        FieldFsXl(self)
    }
}
pub struct FieldHpLpf2XlBw<'a>(pub &'a mut Ctrl8Val);
impl<'a> FieldHpLpf2XlBw<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl8Val {
        self.0.0 &= !(!(!0 << 3) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 5;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl8Val {
        self.0.0 &= !(!(!0 << 3) << 5);
        self.0.0 |= 0x0 & (!(!0 << 3) << 5);
        self.0
    }
}
pub struct FieldFsXl<'a>(pub &'a mut Ctrl8Val);
impl<'a> FieldFsXl<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl8Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl8Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x0 & (!(!0 << 2) << 0);
        self.0
    }
}
