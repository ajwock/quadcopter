use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl6<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl6<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl6Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x15, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl6Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl6Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x15, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl6Val(val))
    }
    pub fn write(&mut self, val: Ctrl6Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x15, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl6Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl6Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x15, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl6Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl6Val) -> Ctrl6Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl6Val) -> Ctrl6Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl6Val(0x8))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl6Val(0x8)).await
    }
}
pub struct Ctrl6Val(pub u8);
impl Ctrl6Val {
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
        Self(0x8)
    }
    pub fn lpf1_g_bw<'a>(&'a mut self) -> FieldLpf1GBw<'a> {
        FieldLpf1GBw(self)
    }
    pub fn fs_g<'a>(&'a mut self) -> FieldFsG<'a> {
        FieldFsG(self)
    }
}
pub struct FieldLpf1GBw<'a>(pub &'a mut Ctrl6Val);
impl<'a> FieldLpf1GBw<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl6Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl6Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= 0x8 & (!(!0 << 3) << 4);
        self.0
    }
}
pub struct FieldFsG<'a>(pub &'a mut Ctrl6Val);
impl<'a> FieldFsG<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl6Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl6Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x8 & (!(!0 << 3) << 0);
        self.0
    }
}
