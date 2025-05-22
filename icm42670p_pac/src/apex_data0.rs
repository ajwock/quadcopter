use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexData0<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexData0<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexData0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x31, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x31, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData0Val(val))
    }
}
pub struct ApexData0Val(pub u8);
impl ApexData0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn step_cnt_7_0<'a>(&'a mut self) -> FieldStepCnt70<'a> {
        FieldStepCnt70(self)
    }
}
pub struct FieldStepCnt70<'a>(pub &'a mut ApexData0Val);
impl<'a> FieldStepCnt70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
