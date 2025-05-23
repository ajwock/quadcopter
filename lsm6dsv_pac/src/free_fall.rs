use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FreeFall<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FreeFall<'a, C> {
    pub fn read(&mut self) -> Result<FreeFallVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x5d, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FreeFallVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FreeFallVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x5d, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FreeFallVal(val))
    }
    pub fn write(&mut self, val: FreeFallVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x5d, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FreeFallVal(raw_val))
    }
    pub async fn write_async(&mut self, val: FreeFallVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x5d, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FreeFallVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(FreeFallVal) -> FreeFallVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FreeFallVal) -> FreeFallVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FreeFallVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FreeFallVal(0x0)).await
    }
}
pub struct FreeFallVal(pub u8);
impl FreeFallVal {
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
    pub fn ff_dur_4_0<'a>(&'a mut self) -> FieldFfDur40<'a> {
        FieldFfDur40(self)
    }
    pub fn ff_ths<'a>(&'a mut self) -> FieldFfThs<'a> {
        FieldFfThs(self)
    }
}
pub struct FieldFfDur40<'a>(pub &'a mut FreeFallVal);
impl<'a> FieldFfDur40<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 5)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FreeFallVal {
        self.0.0 &= !(!(!0 << 5) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 5)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut FreeFallVal {
        self.0.0 &= !(!(!0 << 5) << 3);
        self.0.0 |= 0x0 & (!(!0 << 5) << 3);
        self.0
    }
}
pub struct FieldFfThs<'a>(pub &'a mut FreeFallVal);
impl<'a> FieldFfThs<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FreeFallVal {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut FreeFallVal {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x0 & (!(!0 << 3) << 0);
        self.0
    }
}
