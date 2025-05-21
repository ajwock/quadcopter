use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct DriveConfig2<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> DriveConfig2<'a, C> {
    pub fn read(&mut self) -> Result<DriveConfig2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x4, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<DriveConfig2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x4, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig2Val(val))
    }
    pub fn write(&mut self, val: DriveConfig2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x4, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: DriveConfig2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x4, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct DriveConfig2Val(pub u8);
impl DriveConfig2Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn i2_c_slew_rate<'a>(&'a mut self) -> I2CSlewRate<'a> {
        I2CSlewRate(self)
    }
    pub fn all_slew_rate<'a>(&'a mut self) -> AllSlewRate<'a> {
        AllSlewRate(self)
    }
}
pub struct I2CSlewRate<'a>(pub &'a mut DriveConfig2Val);
impl<'a> I2CSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig2Val {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
}
pub struct AllSlewRate<'a>(pub &'a mut DriveConfig2Val);
impl<'a> AllSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig2Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
}
