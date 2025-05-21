use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct TempData<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> TempData<'a, C> {
    pub fn read(&mut self) -> Result<TempDataVal, RegCommsError> {
        let mut buf = [0u8; 2];
        self.0.comms_read(0x9, &mut buf, crate::AccessProc::Standard)?;
        let val = u16::from_be_bytes(buf);
        Ok(TempDataVal(val))
    }
    pub async fn read_async(&mut self) -> Result<TempDataVal, RegCommsError> {
        let mut buf = [0u8; 2];
        self.0.comms_read_async(0x9, &mut buf, crate::AccessProc::Standard).await?;
        let val = u16::from_be_bytes(buf);
        Ok(TempDataVal(val))
    }
}
pub struct TempDataVal(pub u16);
impl TempDataVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
