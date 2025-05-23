use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct HgFunctionsEnable<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> HgFunctionsEnable<'a, C> {
    pub fn read(&mut self) -> Result<HgFunctionsEnableVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x52, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(HgFunctionsEnableVal(val))
    }
    pub async fn read_async(&mut self) -> Result<HgFunctionsEnableVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x52, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(HgFunctionsEnableVal(val))
    }
    pub fn write(&mut self, val: HgFunctionsEnableVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x52, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(HgFunctionsEnableVal(raw_val))
    }
    pub async fn write_async(&mut self, val: HgFunctionsEnableVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x52, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(HgFunctionsEnableVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(HgFunctionsEnableVal) -> HgFunctionsEnableVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(HgFunctionsEnableVal) -> HgFunctionsEnableVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(HgFunctionsEnableVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(HgFunctionsEnableVal(0x0)).await
    }
}
pub struct HgFunctionsEnableVal(pub u8);
impl HgFunctionsEnableVal {
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
    pub fn hg_interrupts_enable<'a>(&'a mut self) -> FieldHgInterruptsEnable<'a> {
        FieldHgInterruptsEnable(self)
    }
    pub fn hg_wu_change_int_sel<'a>(&'a mut self) -> FieldHgWuChangeIntSel<'a> {
        FieldHgWuChangeIntSel(self)
    }
    pub fn int2_hg_wu<'a>(&'a mut self) -> FieldInt2HgWu<'a> {
        FieldInt2HgWu(self)
    }
    pub fn int1_hg_wu<'a>(&'a mut self) -> FieldInt1HgWu<'a> {
        FieldInt1HgWu(self)
    }
    pub fn hg_shock_dur<'a>(&'a mut self) -> FieldHgShockDur<'a> {
        FieldHgShockDur(self)
    }
}
pub struct FieldHgInterruptsEnable<'a>(pub &'a mut HgFunctionsEnableVal);
impl<'a> FieldHgInterruptsEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgFunctionsEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgFunctionsEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldHgWuChangeIntSel<'a>(pub &'a mut HgFunctionsEnableVal);
impl<'a> FieldHgWuChangeIntSel<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgFunctionsEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgFunctionsEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldInt2HgWu<'a>(pub &'a mut HgFunctionsEnableVal);
impl<'a> FieldInt2HgWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgFunctionsEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgFunctionsEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt1HgWu<'a>(pub &'a mut HgFunctionsEnableVal);
impl<'a> FieldInt1HgWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgFunctionsEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgFunctionsEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldHgShockDur<'a>(pub &'a mut HgFunctionsEnableVal);
impl<'a> FieldHgShockDur<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut HgFunctionsEnableVal {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x0 & (!(!0 << 4) << 0);
        self.0
    }
}
