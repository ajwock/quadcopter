use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig12<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexConfig12<'a, C> {
    pub fn read(&mut self) -> Result<ApexConfig12Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x67, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig12Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig12Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x67, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig12Val(val))
    }
    pub fn write(&mut self, val: ApexConfig12Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x67, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig12Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig12Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x67, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig12Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig12Val) -> ApexConfig12Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig12Val) -> ApexConfig12Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig12Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig12Val(0x0)).await
    }
}
pub struct ApexConfig12Val(pub u8);
impl ApexConfig12Val {
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
    pub fn ff_max_duration_sel<'a>(&'a mut self) -> FieldFfMaxDurationSel<'a> {
        FieldFfMaxDurationSel(self)
    }
    pub fn ff_min_duration_sel<'a>(&'a mut self) -> FieldFfMinDurationSel<'a> {
        FieldFfMinDurationSel(self)
    }
}
pub struct FieldFfMaxDurationSel<'a>(pub &'a mut ApexConfig12Val);
impl<'a> FieldFfMaxDurationSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig12Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig12Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= 0x0 & (!(!0 << 4) << 4);
        self.0
    }
}
pub struct FieldFfMinDurationSel<'a>(pub &'a mut ApexConfig12Val);
impl<'a> FieldFfMinDurationSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig12Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig12Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x0 & (!(!0 << 4) << 0);
        self.0
    }
}
