use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexData1<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexData1<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexData1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x32, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x32, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData1Val(val))
    }
}
pub struct ApexData1Val(pub u8);
impl ApexData1Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn step_cnt_15_8<'a>(&'a mut self) -> FieldStepCnt158<'a> {
        FieldStepCnt158(self)
    }
}
pub struct FieldStepCnt158<'a>(pub &'a mut ApexData1Val);
impl<'a> FieldStepCnt158<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
