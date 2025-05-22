use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct AccelConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> AccelConfig1<'a, C> {
    pub fn read(&mut self) -> Result<AccelConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x24, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<AccelConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x24, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelConfig1Val(val))
    }
    pub fn write(&mut self, val: AccelConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x24, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(AccelConfig1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: AccelConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x24, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(AccelConfig1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(AccelConfig1Val) -> AccelConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(AccelConfig1Val) -> AccelConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(AccelConfig1Val(0x41))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(AccelConfig1Val(0x41)).await
    }
}
pub struct AccelConfig1Val(pub u8);
impl AccelConfig1Val {
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
        Self(0x41)
    }
    pub fn accel_ui_avg<'a>(&'a mut self) -> FieldAccelUiAvg<'a> {
        FieldAccelUiAvg(self)
    }
    pub fn accel_ui_filt_bw<'a>(&'a mut self) -> FieldAccelUiFiltBw<'a> {
        FieldAccelUiFiltBw(self)
    }
}
pub struct FieldAccelUiAvg<'a>(pub &'a mut AccelConfig1Val);
impl<'a> FieldAccelUiAvg<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut AccelConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut AccelConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= 0x41 & (!(!0 << 3) << 4);
        self.0
    }
}
pub struct FieldAccelUiFiltBw<'a>(pub &'a mut AccelConfig1Val);
impl<'a> FieldAccelUiFiltBw<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut AccelConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut AccelConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x41 & (!(!0 << 3) << 0);
        self.0
    }
}
