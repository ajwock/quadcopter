use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct DriveConfig3<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> DriveConfig3<'a, C> {
    pub fn read(&mut self) -> Result<DriveConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x5, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<DriveConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x5, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(DriveConfig3Val(val))
    }
    pub fn write(&mut self, val: DriveConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x5, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(DriveConfig3Val(raw_val))
    }
    pub async fn write_async(&mut self, val: DriveConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x5, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(DriveConfig3Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(DriveConfig3Val) -> DriveConfig3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(DriveConfig3Val) -> DriveConfig3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(DriveConfig3Val(0x5))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(DriveConfig3Val(0x5)).await
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
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0x5)
    }
    pub fn spi_slew_rate<'a>(&'a mut self) -> FieldSpiSlewRate<'a> {
        FieldSpiSlewRate(self)
    }
}
pub struct FieldSpiSlewRate<'a>(pub &'a mut DriveConfig3Val);
impl<'a> FieldSpiSlewRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut DriveConfig3Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut DriveConfig3Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x5 & (!(!0 << 3) << 0);
        self.0
    }
}
