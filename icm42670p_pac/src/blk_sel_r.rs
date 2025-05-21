use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct BlkSelR<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> BlkSelR<'a, C> {
    pub fn read(&mut self) -> Result<BlkSelRVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x7c, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(BlkSelRVal(val))
    }
    pub async fn read_async(&mut self) -> Result<BlkSelRVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x7c, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(BlkSelRVal(val))
    }
}
pub struct BlkSelRVal(pub u8);
impl BlkSelRVal {
    pub fn get(&self) -> u8 {
        self.0
    }
}
