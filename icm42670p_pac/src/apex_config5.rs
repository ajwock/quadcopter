use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig5<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexConfig5<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexConfig5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x47, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig5Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x47, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig5Val(val))
    }
    pub fn write(&mut self, val: ApexConfig5Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x47, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig5Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig5Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x47, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig5Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig5Val) -> ApexConfig5Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig5Val) -> ApexConfig5Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig5Val(0x80))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig5Val(0x80)).await
    }
}
pub struct ApexConfig5Val(pub u8);
impl ApexConfig5Val {
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
        Self(0x80)
    }
    pub fn tilt_wait_time_sel<'a>(&'a mut self) -> FieldTiltWaitTimeSel<'a> {
        FieldTiltWaitTimeSel(self)
    }
    pub fn lowg_peak_th_hyst_sel<'a>(&'a mut self) -> FieldLowgPeakThHystSel<'a> {
        FieldLowgPeakThHystSel(self)
    }
    pub fn highg_peak_th_hyst_sel<'a>(&'a mut self) -> FieldHighgPeakThHystSel<'a> {
        FieldHighgPeakThHystSel(self)
    }
}
pub struct FieldTiltWaitTimeSel<'a>(pub &'a mut ApexConfig5Val);
impl<'a> FieldTiltWaitTimeSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 6) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig5Val {
        self.0.0 &= !(!(!0 << 2) << 6);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 6;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig5Val {
        self.0.0 &= !(!(!0 << 2) << 6);
        self.0.0 |= 0x80 & (!(!0 << 2) << 6);
        self.0
    }
}
pub struct FieldLowgPeakThHystSel<'a>(pub &'a mut ApexConfig5Val);
impl<'a> FieldLowgPeakThHystSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig5Val {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig5Val {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= 0x80 & (!(!0 << 3) << 3);
        self.0
    }
}
pub struct FieldHighgPeakThHystSel<'a>(pub &'a mut ApexConfig5Val);
impl<'a> FieldHighgPeakThHystSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig5Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig5Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x80 & (!(!0 << 3) << 0);
        self.0
    }
}
