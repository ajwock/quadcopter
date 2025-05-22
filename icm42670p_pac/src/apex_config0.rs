use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig0<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexConfig0<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x25, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x25, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig0Val(val))
    }
    pub fn write(&mut self, val: ApexConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x25, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x25, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig0Val) -> ApexConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig0Val) -> ApexConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig0Val(0x8))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig0Val(0x8)).await
    }
}
pub struct ApexConfig0Val(pub u8);
impl ApexConfig0Val {
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
        Self(0x8)
    }
    pub fn dmp_power_save_en<'a>(&'a mut self) -> FieldDmpPowerSaveEn<'a> {
        FieldDmpPowerSaveEn(self)
    }
    pub fn dmp_init_en<'a>(&'a mut self) -> FieldDmpInitEn<'a> {
        FieldDmpInitEn(self)
    }
    pub fn dmp_mem_reset_en<'a>(&'a mut self) -> FieldDmpMemResetEn<'a> {
        FieldDmpMemResetEn(self)
    }
}
pub struct FieldDmpPowerSaveEn<'a>(pub &'a mut ApexConfig0Val);
impl<'a> FieldDmpPowerSaveEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x8;
        self.0
    }
}
pub struct FieldDmpInitEn<'a>(pub &'a mut ApexConfig0Val);
impl<'a> FieldDmpInitEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x8;
        self.0
    }
}
pub struct FieldDmpMemResetEn<'a>(pub &'a mut ApexConfig0Val);
impl<'a> FieldDmpMemResetEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x8;
        self.0
    }
}
