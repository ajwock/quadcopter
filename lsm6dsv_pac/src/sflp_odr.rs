use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct SflpOdr<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> SflpOdr<'a, C> {
    pub fn read(&mut self) -> Result<SflpOdrVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x5e, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(SflpOdrVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SflpOdrVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x5e, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(SflpOdrVal(val))
    }
    pub fn write(&mut self, val: SflpOdrVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x5e, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(SflpOdrVal(raw_val))
    }
    pub async fn write_async(&mut self, val: SflpOdrVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x5e, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(SflpOdrVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(SflpOdrVal) -> SflpOdrVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(SflpOdrVal) -> SflpOdrVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(SflpOdrVal(0x5b))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(SflpOdrVal(0x5b)).await
    }
}
pub struct SflpOdrVal(pub u8);
impl SflpOdrVal {
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
        Self(0x5b)
    }
    pub fn sflp_game_odr<'a>(&'a mut self) -> FieldSflpGameOdr<'a> {
        FieldSflpGameOdr(self)
    }
}
pub struct FieldSflpGameOdr<'a>(pub &'a mut SflpOdrVal);
impl<'a> FieldSflpGameOdr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut SflpOdrVal {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut SflpOdrVal {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= 0x5b & (!(!0 << 3) << 3);
        self.0
    }
}
