use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct ApexData1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexData1<'a, C> {
    pub fn read(&mut self) -> Result<ApexData1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x32, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x32, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData1Val(val))
    }
}
pub struct ApexData1Val(pub u8);
impl ApexData1Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn step_cnt_15_8<'a>(&'a mut self) -> StepCnt158<'a> {
        StepCnt158(self)
    }
}
pub struct StepCnt158<'a>(pub &'a mut ApexData1Val);
impl<'a> StepCnt158<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
