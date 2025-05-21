use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct ApexData4<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexData4<'a, C> {
    pub fn read(&mut self) -> Result<ApexData4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x1d, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData4Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x1d, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData4Val(val))
    }
}
pub struct ApexData4Val(pub u8);
impl ApexData4Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn ff_dur_7_0<'a>(&'a mut self) -> FfDur70<'a> {
        FfDur70(self)
    }
}
pub struct FfDur70<'a>(pub &'a mut ApexData4Val);
impl<'a> FfDur70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
