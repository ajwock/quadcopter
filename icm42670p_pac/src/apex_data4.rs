use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexData4<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexData4<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexData4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1d, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData4Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1d, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData4Val(val))
    }
}
pub struct ApexData4Val(pub u8);
impl ApexData4Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn ff_dur_7_0<'a>(&'a mut self) -> FieldFfDur70<'a> {
        FieldFfDur70(self)
    }
}
pub struct FieldFfDur70<'a>(pub &'a mut ApexData4Val);
impl<'a> FieldFfDur70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
