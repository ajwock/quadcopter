use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct DriveConfig3<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> DriveConfig3<'a, C> {
    pub fn read(&mut self) -> Result<DriveConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x5, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<DriveConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x5, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig3Val(val))
    }
    pub fn write(&mut self, val: DriveConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x5, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: DriveConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x5, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct DriveConfig3Val(pub u8);
impl DriveConfig3Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn spi_slew_rate<'a>(&'a mut self) -> SpiSlewRate<'a> {
        SpiSlewRate(self)
    }
}
pub struct SpiSlewRate<'a>(pub &'a mut DriveConfig3Val);
impl<'a> SpiSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig3Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
}
