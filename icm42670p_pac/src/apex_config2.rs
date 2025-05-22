use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig2<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexConfig2<'a, C> {
    pub fn read(&mut self) -> Result<ApexConfig2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x44, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x44, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig2Val(val))
    }
    pub fn write(&mut self, val: ApexConfig2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x44, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig2Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x44, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig2Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig2Val) -> ApexConfig2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig2Val) -> ApexConfig2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig2Val(0xa2))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig2Val(0xa2)).await
    }
}
pub struct ApexConfig2Val(pub u8);
impl ApexConfig2Val {
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
        Self(0xa2)
    }
    pub fn low_energy_amp_th_sel<'a>(&'a mut self) -> FieldLowEnergyAmpThSel<'a> {
        FieldLowEnergyAmpThSel(self)
    }
    pub fn dmp_power_save_time_sel<'a>(&'a mut self) -> FieldDmpPowerSaveTimeSel<'a> {
        FieldDmpPowerSaveTimeSel(self)
    }
}
pub struct FieldLowEnergyAmpThSel<'a>(pub &'a mut ApexConfig2Val);
impl<'a> FieldLowEnergyAmpThSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig2Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig2Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= 0xa2 & (!(!0 << 4) << 4);
        self.0
    }
}
pub struct FieldDmpPowerSaveTimeSel<'a>(pub &'a mut ApexConfig2Val);
impl<'a> FieldDmpPowerSaveTimeSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig2Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig2Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0xa2 & (!(!0 << 4) << 0);
        self.0
    }
}
