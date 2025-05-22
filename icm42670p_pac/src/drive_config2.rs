use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct DriveConfig2<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> DriveConfig2<'a, D, C> {
    pub fn read(&mut self) -> Result<DriveConfig2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x4, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<DriveConfig2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x4, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig2Val(val))
    }
    pub fn write(&mut self, val: DriveConfig2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x4, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(DriveConfig2Val(raw_val))
    }
    pub async fn write_async(&mut self, val: DriveConfig2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x4, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(DriveConfig2Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(DriveConfig2Val) -> DriveConfig2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(DriveConfig2Val) -> DriveConfig2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(DriveConfig2Val(0xd))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(DriveConfig2Val(0xd)).await
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
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0xd)
    }
    pub fn i2_c_slew_rate<'a>(&'a mut self) -> FieldI2CSlewRate<'a> {
        FieldI2CSlewRate(self)
    }
    pub fn all_slew_rate<'a>(&'a mut self) -> FieldAllSlewRate<'a> {
        FieldAllSlewRate(self)
    }
}
pub struct FieldI2CSlewRate<'a>(pub &'a mut DriveConfig2Val);
impl<'a> FieldI2CSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig2Val {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut DriveConfig2Val {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= 0xd & (!(!0 << 3) << 3);
        self.0
    }
}
pub struct FieldAllSlewRate<'a>(pub &'a mut DriveConfig2Val);
impl<'a> FieldAllSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig2Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut DriveConfig2Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0xd & (!(!0 << 3) << 0);
        self.0
    }
}
