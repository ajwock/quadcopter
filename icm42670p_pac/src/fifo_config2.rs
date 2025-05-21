use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct FifoConfig2<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> FifoConfig2<'a, C> {
    pub fn read(&mut self) -> Result<FifoConfig2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x29, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoConfig2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x29, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig2Val(val))
    }
    pub fn write(&mut self, val: FifoConfig2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x29, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: FifoConfig2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x29, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct FifoConfig2Val(pub u8);
impl FifoConfig2Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn fifo_wm_7_0<'a>(&'a mut self) -> FifoWm70<'a> {
        FifoWm70(self)
    }
}
pub struct FifoWm70<'a>(pub &'a mut FifoConfig2Val);
impl<'a> FifoWm70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
    pub fn set(self, val: u8) -> &'a mut FifoConfig2Val {
        self.0.0 = val;
        self.0
    }
}
