use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct MclkRdy<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> MclkRdy<'a, C> {
    pub fn read(&mut self) -> Result<MclkRdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x0, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(MclkRdyVal(val))
    }
    pub async fn read_async(&mut self) -> Result<MclkRdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x0, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(MclkRdyVal(val))
    }
}
pub struct MclkRdyVal(pub u8);
impl MclkRdyVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn mclk_rdy<'a>(&'a mut self) -> MclkRdy<'a> {
        MclkRdy(self)
    }
}
pub struct MclkRdy<'a>(pub &'a mut MclkRdyVal);
impl<'a> MclkRdy<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
