use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct FdrConfig<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> FdrConfig<'a, D, C> {
    pub fn read(&mut self) -> Result<FdrConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x66, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(FdrConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FdrConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x66, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FdrConfigVal(val))
    }
    pub fn write(&mut self, val: FdrConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x66, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FdrConfigVal(raw_val))
    }
    pub async fn write_async(&mut self, val: FdrConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x66, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FdrConfigVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(FdrConfigVal) -> FdrConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FdrConfigVal) -> FdrConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FdrConfigVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FdrConfigVal(0x0)).await
    }
}
pub struct FdrConfigVal(pub u8);
impl FdrConfigVal {
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
        Self(0x0)
    }
    pub fn fdr_sel<'a>(&'a mut self) -> FieldFdrSel<'a> {
        FieldFdrSel(self)
    }
}
pub struct FieldFdrSel<'a>(pub &'a mut FdrConfigVal);
impl<'a> FieldFdrSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FdrConfigVal {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut FdrConfigVal {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x0 & (!(!0 << 4) << 0);
        self.0
    }
}
