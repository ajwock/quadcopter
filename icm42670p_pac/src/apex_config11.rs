use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig11<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexConfig11<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexConfig11Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x4a, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig11Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig11Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x4a, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig11Val(val))
    }
    pub fn write(&mut self, val: ApexConfig11Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x4a, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig11Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig11Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x4a, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig11Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig11Val) -> ApexConfig11Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig11Val) -> ApexConfig11Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig11Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig11Val(0x0)).await
    }
}
pub struct ApexConfig11Val(pub u8);
impl ApexConfig11Val {
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
    pub fn highg_peak_th_sel<'a>(&'a mut self) -> FieldHighgPeakThSel<'a> {
        FieldHighgPeakThSel(self)
    }
    pub fn highg_time_th_sel<'a>(&'a mut self) -> FieldHighgTimeThSel<'a> {
        FieldHighgTimeThSel(self)
    }
}
pub struct FieldHighgPeakThSel<'a>(pub &'a mut ApexConfig11Val);
impl<'a> FieldHighgPeakThSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 5)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig11Val {
        self.0.0 &= !(!(!0 << 5) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 5)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig11Val {
        self.0.0 &= !(!(!0 << 5) << 3);
        self.0.0 |= 0x0 & (!(!0 << 5) << 3);
        self.0
    }
}
pub struct FieldHighgTimeThSel<'a>(pub &'a mut ApexConfig11Val);
impl<'a> FieldHighgTimeThSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig11Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig11Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x0 & (!(!0 << 3) << 0);
        self.0
    }
}
