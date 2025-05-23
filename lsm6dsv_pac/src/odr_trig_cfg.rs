use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct OdrTrigCfg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> OdrTrigCfg<'a, C> {
    pub fn read(&mut self) -> Result<OdrTrigCfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x6, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(OdrTrigCfgVal(val))
    }
    pub async fn read_async(&mut self) -> Result<OdrTrigCfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x6, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(OdrTrigCfgVal(val))
    }
    pub fn write(&mut self, val: OdrTrigCfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x6, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(OdrTrigCfgVal(raw_val))
    }
    pub async fn write_async(&mut self, val: OdrTrigCfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x6, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(OdrTrigCfgVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(OdrTrigCfgVal) -> OdrTrigCfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(OdrTrigCfgVal) -> OdrTrigCfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(OdrTrigCfgVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(OdrTrigCfgVal(0x0)).await
    }
}
pub struct OdrTrigCfgVal(pub u8);
impl OdrTrigCfgVal {
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
