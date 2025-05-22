use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig9<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexConfig9<'a, C> {
    pub fn read(&mut self) -> Result<ApexConfig9Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x48, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig9Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig9Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x48, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig9Val(val))
    }
    pub fn write(&mut self, val: ApexConfig9Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x48, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig9Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig9Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x48, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig9Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig9Val) -> ApexConfig9Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig9Val) -> ApexConfig9Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig9Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig9Val(0x0)).await
    }
}
pub struct ApexConfig9Val(pub u8);
impl ApexConfig9Val {
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
    pub fn ff_debounce_duration_sel<'a>(&'a mut self) -> FieldFfDebounceDurationSel<'a> {
        FieldFfDebounceDurationSel(self)
    }
    pub fn smd_sensitivity_sel<'a>(&'a mut self) -> FieldSmdSensitivitySel<'a> {
        FieldSmdSensitivitySel(self)
    }
    pub fn sensitivity_mode<'a>(&'a mut self) -> FieldSensitivityMode<'a> {
        FieldSensitivityMode(self)
    }
}
pub struct FieldFfDebounceDurationSel<'a>(pub &'a mut ApexConfig9Val);
impl<'a> FieldFfDebounceDurationSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig9Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig9Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= 0x0 & (!(!0 << 4) << 4);
        self.0
    }
}
pub struct FieldSmdSensitivitySel<'a>(pub &'a mut ApexConfig9Val);
impl<'a> FieldSmdSensitivitySel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 1) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig9Val {
        self.0.0 &= !(!(!0 << 3) << 1);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 1;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig9Val {
        self.0.0 &= !(!(!0 << 3) << 1);
        self.0.0 |= 0x0 & (!(!0 << 3) << 1);
        self.0
    }
}
pub struct FieldSensitivityMode<'a>(pub &'a mut ApexConfig9Val);
impl<'a> FieldSensitivityMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig9Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut ApexConfig9Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
