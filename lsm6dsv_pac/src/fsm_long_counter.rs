use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FsmLongCounter<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FsmLongCounter<'a, C> {
    pub fn read(&mut self) -> Result<FsmLongCounterVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x48, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(FsmLongCounterVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FsmLongCounterVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x48, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(FsmLongCounterVal(val))
    }
    pub fn write(&mut self, val: FsmLongCounterVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x48, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u16) -> Result<(), RegCommsError> {
        self.write(FsmLongCounterVal(raw_val))
    }
    pub async fn write_async(&mut self, val: FsmLongCounterVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x48, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u16) -> Result<(), RegCommsError> {
        self.write_async(FsmLongCounterVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(FsmLongCounterVal) -> FsmLongCounterVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FsmLongCounterVal) -> FsmLongCounterVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FsmLongCounterVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FsmLongCounterVal(0x0)).await
    }
}
pub struct FsmLongCounterVal(pub u16);
impl FsmLongCounterVal {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn zero() -> Self {
        Self(0)
    }
    pub fn set(&mut self, val: u16) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
}
