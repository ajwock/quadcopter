use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct WhoAmI<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> WhoAmI<'a, D, C> {
    pub fn read(&mut self) -> Result<WhoAmIVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x75, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(WhoAmIVal(val))
    }
    pub async fn read_async(&mut self) -> Result<WhoAmIVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x75, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(WhoAmIVal(val))
    }
}
pub struct WhoAmIVal(pub u8);
impl WhoAmIVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x67)
    }
    pub fn whoami<'a>(&'a mut self) -> FieldWhoami<'a> {
        FieldWhoami(self)
    }
}
pub struct FieldWhoami<'a>(pub &'a mut WhoAmIVal);
impl<'a> FieldWhoami<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
