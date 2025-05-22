use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct BlkSelW<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> BlkSelW<'a, D, C> {
    pub fn read(&mut self) -> Result<BlkSelWVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x79, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(BlkSelWVal(val))
    }
    pub async fn read_async(&mut self) -> Result<BlkSelWVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x79, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(BlkSelWVal(val))
    }
    pub fn write(&mut self, val: BlkSelWVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x79, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(BlkSelWVal(raw_val))
    }
    pub async fn write_async(&mut self, val: BlkSelWVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x79, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(BlkSelWVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(BlkSelWVal) -> BlkSelWVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(BlkSelWVal) -> BlkSelWVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(BlkSelWVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(BlkSelWVal(0x0)).await
    }
}
pub struct BlkSelWVal(pub u8);
impl BlkSelWVal {
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
