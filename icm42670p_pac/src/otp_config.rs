use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct OtpConfig<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> OtpConfig<'a, D, C> {
    pub fn read(&mut self) -> Result<OtpConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x2b, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(OtpConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<OtpConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x2b, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(OtpConfigVal(val))
    }
    pub fn write(&mut self, val: OtpConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x2b, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(OtpConfigVal(raw_val))
    }
    pub async fn write_async(&mut self, val: OtpConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x2b, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(OtpConfigVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(OtpConfigVal) -> OtpConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(OtpConfigVal) -> OtpConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(OtpConfigVal(0x6))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(OtpConfigVal(0x6)).await
    }
}
pub struct OtpConfigVal(pub u8);
impl OtpConfigVal {
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
        Self(0x6)
    }
    pub fn otp_copy_mode<'a>(&'a mut self) -> FieldOtpCopyMode<'a> {
        FieldOtpCopyMode(self)
    }
}
pub struct FieldOtpCopyMode<'a>(pub &'a mut OtpConfigVal);
impl<'a> FieldOtpCopyMode<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut OtpConfigVal {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 2;
        self.0
    }
    pub fn reset(self) -> &'a mut OtpConfigVal {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= 0x6 & (!(!0 << 2) << 2);
        self.0
    }
}
