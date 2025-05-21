use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct DriveConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> DriveConfig1<'a, C> {
    pub fn read(&mut self) -> Result<DriveConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x3, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<DriveConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x3, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig1Val(val))
    }
    pub fn write(&mut self, val: DriveConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x3, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: DriveConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x3, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct DriveConfig1Val(pub u8);
impl DriveConfig1Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn i3_c_ddr_slew_rate<'a>(&'a mut self) -> I3CDdrSlewRate<'a> {
        I3CDdrSlewRate(self)
    }
    pub fn i3_c_sdr_slew_rate<'a>(&'a mut self) -> I3CSdrSlewRate<'a> {
        I3CSdrSlewRate(self)
    }
}
pub struct I3CDdrSlewRate<'a>(pub &'a mut DriveConfig1Val);
impl<'a> I3CDdrSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
}
pub struct I3CSdrSlewRate<'a>(pub &'a mut DriveConfig1Val);
impl<'a> I3CSdrSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
}
