use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct MR<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> MR<'a, C> {
    pub fn read(&mut self) -> Result<MRVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x7e, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(MRVal(val))
    }
    pub async fn read_async(&mut self) -> Result<MRVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x7e, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(MRVal(val))
    }
    pub fn write(&mut self, val: MRVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x7e, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: MRVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x7e, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct MRVal(pub u8);
impl MRVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
}
