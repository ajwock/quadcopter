use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct Rdy<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> Rdy<'a, C> {
    pub fn read(&mut self) -> Result<RdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x0, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(RdyVal(val))
    }
    pub async fn read_async(&mut self) -> Result<RdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x0, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(RdyVal(val))
    }
}
pub struct RdyVal(pub u8);
impl RdyVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn mclk_rdy<'a>(&'a mut self) -> MclkRdy<'a> {
        MclkRdy(self)
    }
}
pub struct MclkRdy<'a>(pub &'a mut RdyVal);
impl<'a> MclkRdy<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
