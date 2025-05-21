use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct GyroDataX<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> GyroDataX<'a, C> {
    pub fn read(&mut self) -> Result<GyroDataXVal, RegCommsError> {
        let mut buf = [0u8; 2];
        self.0.comms_read(0x11, &mut buf, crate::AccessProc::Standard)?;
        let val = u16::from_be_bytes(buf);
        Ok(GyroDataXVal(val))
    }
    pub async fn read_async(&mut self) -> Result<GyroDataXVal, RegCommsError> {
        let mut buf = [0u8; 2];
        self.0.comms_read_async(0x11, &mut buf, crate::AccessProc::Standard).await?;
        let val = u16::from_be_bytes(buf);
        Ok(GyroDataXVal(val))
    }
}
pub struct GyroDataXVal(pub u16);
impl GyroDataXVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
