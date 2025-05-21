use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct ApexData0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexData0<'a, C> {
    pub fn read(&mut self) -> Result<ApexData0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x31, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x31, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData0Val(val))
    }
}
pub struct ApexData0Val(pub u8);
impl ApexData0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn step_cnt_7_0<'a>(&'a mut self) -> StepCnt70<'a> {
        StepCnt70(self)
    }
}
pub struct StepCnt70<'a>(pub &'a mut ApexData0Val);
impl<'a> StepCnt70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
