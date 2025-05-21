use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct MaddrR<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> MaddrR<'a, C> {
    pub fn read(&mut self) -> Result<MaddrRVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x7d, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(MaddrRVal(val))
    }
    pub async fn read_async(&mut self) -> Result<MaddrRVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x7d, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(MaddrRVal(val))
    }
    pub fn write(&mut self, val: MaddrRVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x7d, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: MaddrRVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x7d, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct MaddrRVal(pub u8);
impl MaddrRVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
}
