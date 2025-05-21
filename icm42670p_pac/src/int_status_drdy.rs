use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntStatusDrdy<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntStatusDrdy<'a, C> {
    pub fn read(&mut self) -> Result<IntStatusDrdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x39, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatusDrdyVal(val))
    }
    pub async fn read_async(&mut self) -> Result<IntStatusDrdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x39, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatusDrdyVal(val))
    }
}
pub struct IntStatusDrdyVal(pub u8);
impl IntStatusDrdyVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn data_rdy_int<'a>(&'a mut self) -> DataRdyInt<'a> {
        DataRdyInt(self)
    }
}
pub struct DataRdyInt<'a>(pub &'a mut IntStatusDrdyVal);
impl<'a> DataRdyInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
