use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct ApexData2<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexData2<'a, C> {
    pub fn read(&mut self) -> Result<ApexData2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x33, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x33, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData2Val(val))
    }
}
pub struct ApexData2Val(pub u8);
impl ApexData2Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn step_cadence<'a>(&'a mut self) -> StepCadence<'a> {
        StepCadence(self)
    }
}
pub struct StepCadence<'a>(pub &'a mut ApexData2Val);
impl<'a> StepCadence<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
