use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct ZOfsUsr<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> ZOfsUsr<'a, C> {
    pub fn read(&mut self) -> Result<ZOfsUsrVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x75, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(ZOfsUsrVal(val))
    }
    pub async fn read_async(&mut self) -> Result<ZOfsUsrVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x75, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(ZOfsUsrVal(val))
    }
    pub fn write(&mut self, val: ZOfsUsrVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x75, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ZOfsUsrVal(raw_val))
    }
    pub async fn write_async(&mut self, val: ZOfsUsrVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x75, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ZOfsUsrVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(ZOfsUsrVal) -> ZOfsUsrVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ZOfsUsrVal) -> ZOfsUsrVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ZOfsUsrVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ZOfsUsrVal(0x0)).await
    }
}
pub struct ZOfsUsrVal(pub u8);
impl ZOfsUsrVal {
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
}
