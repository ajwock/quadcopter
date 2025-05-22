use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexData5<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexData5<'a, C> {
    pub fn read(&mut self) -> Result<ApexData5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1e, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData5Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1e, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData5Val(val))
    }
}
pub struct ApexData5Val(pub u8);
impl ApexData5Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn ff_dur_15_8<'a>(&'a mut self) -> FieldFfDur158<'a> {
        FieldFfDur158(self)
    }
}
pub struct FieldFfDur158<'a>(pub &'a mut ApexData5Val);
impl<'a> FieldFfDur158<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
