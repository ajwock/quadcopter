use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FunctionsEnable<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FunctionsEnable<'a, C> {
    pub fn read(&mut self) -> Result<FunctionsEnableVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x50, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FunctionsEnableVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FunctionsEnableVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x50, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FunctionsEnableVal(val))
    }
    pub fn write(&mut self, val: FunctionsEnableVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x50, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FunctionsEnableVal(raw_val))
    }
    pub async fn write_async(&mut self, val: FunctionsEnableVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x50, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FunctionsEnableVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(FunctionsEnableVal) -> FunctionsEnableVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FunctionsEnableVal) -> FunctionsEnableVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FunctionsEnableVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FunctionsEnableVal(0x0)).await
    }
}
pub struct FunctionsEnableVal(pub u8);
impl FunctionsEnableVal {
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
    pub fn interrupts_enable<'a>(&'a mut self) -> FieldInterruptsEnable<'a> {
        FieldInterruptsEnable(self)
    }
    pub fn timestamp_en<'a>(&'a mut self) -> FieldTimestampEn<'a> {
        FieldTimestampEn(self)
    }
    pub fn dis_rst_lir_all_int<'a>(&'a mut self) -> FieldDisRstLirAllInt<'a> {
        FieldDisRstLirAllInt(self)
    }
    pub fn inact_en<'a>(&'a mut self) -> FieldInactEn<'a> {
        FieldInactEn(self)
    }
}
pub struct FieldInterruptsEnable<'a>(pub &'a mut FunctionsEnableVal);
impl<'a> FieldInterruptsEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FunctionsEnableVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FunctionsEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FunctionsEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FunctionsEnableVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldTimestampEn<'a>(pub &'a mut FunctionsEnableVal);
impl<'a> FieldTimestampEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FunctionsEnableVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FunctionsEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FunctionsEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FunctionsEnableVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldDisRstLirAllInt<'a>(pub &'a mut FunctionsEnableVal);
impl<'a> FieldDisRstLirAllInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FunctionsEnableVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FunctionsEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FunctionsEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FunctionsEnableVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldInactEn<'a>(pub &'a mut FunctionsEnableVal);
impl<'a> FieldInactEn<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FunctionsEnableVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut FunctionsEnableVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x0 & (!(!0 << 2) << 0);
        self.0
    }
}
