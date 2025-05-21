use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct ApexData5<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexData5<'a, C> {
    pub fn read(&mut self) -> Result<ApexData5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x1e, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData5Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x1e, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData5Val(val))
    }
}
pub struct ApexData5Val(pub u8);
impl ApexData5Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn ff_dur_15_8<'a>(&'a mut self) -> FfDur158<'a> {
        FfDur158(self)
    }
}
pub struct FfDur158<'a>(pub &'a mut ApexData5Val);
impl<'a> FfDur158<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
