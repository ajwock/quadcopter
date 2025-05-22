use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct DriveConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> DriveConfig1<'a, C> {
    pub fn read(&mut self) -> Result<DriveConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x3, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<DriveConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x3, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig1Val(val))
    }
    pub fn write(&mut self, val: DriveConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x3, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(DriveConfig1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: DriveConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x3, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(DriveConfig1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(DriveConfig1Val) -> DriveConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(DriveConfig1Val) -> DriveConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(DriveConfig1Val(0x2b))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(DriveConfig1Val(0x2b)).await
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
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0x2b)
    }
    pub fn i3_c_ddr_slew_rate<'a>(&'a mut self) -> FieldI3CDdrSlewRate<'a> {
        FieldI3CDdrSlewRate(self)
    }
    pub fn i3_c_sdr_slew_rate<'a>(&'a mut self) -> FieldI3CSdrSlewRate<'a> {
        FieldI3CSdrSlewRate(self)
    }
}
pub struct FieldI3CDdrSlewRate<'a>(pub &'a mut DriveConfig1Val);
impl<'a> FieldI3CDdrSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut DriveConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= 0x2b & (!(!0 << 3) << 3);
        self.0
    }
}
pub struct FieldI3CSdrSlewRate<'a>(pub &'a mut DriveConfig1Val);
impl<'a> FieldI3CSdrSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut DriveConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x2b & (!(!0 << 3) << 0);
        self.0
    }
}
