use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct AccelWomZThr<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> AccelWomZThr<'a, C> {
    pub fn read(&mut self) -> Result<AccelWomZThrVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x4d, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelWomZThrVal(val))
    }
    pub async fn read_async(&mut self) -> Result<AccelWomZThrVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x4d, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelWomZThrVal(val))
    }
    pub fn write(&mut self, val: AccelWomZThrVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x4d, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(AccelWomZThrVal(raw_val))
    }
    pub async fn write_async(&mut self, val: AccelWomZThrVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x4d, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(AccelWomZThrVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(AccelWomZThrVal) -> AccelWomZThrVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(AccelWomZThrVal) -> AccelWomZThrVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(AccelWomZThrVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(AccelWomZThrVal(0x0)).await
    }
}
pub struct AccelWomZThrVal(pub u8);
impl AccelWomZThrVal {
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
