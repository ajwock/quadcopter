use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct WhoAmI<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> WhoAmI<'a, C> {
    pub fn write(&mut self, val: WhoAmIVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0xf, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(WhoAmIVal(raw_val))
    }
    pub async fn write_async(&mut self, val: WhoAmIVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0xf, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(WhoAmIVal(raw_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(WhoAmIVal(0x73))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(WhoAmIVal(0x73)).await
    }
}
pub struct WhoAmIVal(pub u8);
impl WhoAmIVal {
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
        Self(0x73)
    }
}
