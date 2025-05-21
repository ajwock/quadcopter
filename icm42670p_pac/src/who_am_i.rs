use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct WhoAmI<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> WhoAmI<'a, C> {
    pub fn read(&mut self) -> Result<WhoAmIVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x75, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(WhoAmIVal(val))
    }
    pub async fn read_async(&mut self) -> Result<WhoAmIVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x75, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(WhoAmIVal(val))
    }
}
pub struct WhoAmIVal(pub u8);
impl WhoAmIVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn whoami<'a>(&'a mut self) -> Whoami<'a> {
        Whoami(self)
    }
}
pub struct Whoami<'a>(pub &'a mut WhoAmIVal);
impl<'a> Whoami<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
