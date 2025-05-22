use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexData2<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexData2<'a, C> {
    pub fn read(&mut self) -> Result<ApexData2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x33, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x33, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData2Val(val))
    }
}
pub struct ApexData2Val(pub u8);
impl ApexData2Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn step_cadence<'a>(&'a mut self) -> FieldStepCadence<'a> {
        FieldStepCadence(self)
    }
}
pub struct FieldStepCadence<'a>(pub &'a mut ApexData2Val);
impl<'a> FieldStepCadence<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
