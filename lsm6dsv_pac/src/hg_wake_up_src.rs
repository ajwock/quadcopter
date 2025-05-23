use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct HgWakeUpSrc<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> HgWakeUpSrc<'a, C> {
    pub fn read(&mut self) -> Result<HgWakeUpSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x4c, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(HgWakeUpSrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<HgWakeUpSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x4c, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(HgWakeUpSrcVal(val))
    }
    pub fn write(&mut self, val: HgWakeUpSrcVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x4c, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(HgWakeUpSrcVal(raw_val))
    }
    pub async fn write_async(&mut self, val: HgWakeUpSrcVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x4c, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(HgWakeUpSrcVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(HgWakeUpSrcVal) -> HgWakeUpSrcVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(HgWakeUpSrcVal) -> HgWakeUpSrcVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(HgWakeUpSrcVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(HgWakeUpSrcVal(0x0)).await
    }
}
pub struct HgWakeUpSrcVal(pub u8);
impl HgWakeUpSrcVal {
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
    pub fn hg_shock_change_ia<'a>(&'a mut self) -> FieldHgShockChangeIa<'a> {
        FieldHgShockChangeIa(self)
    }
    pub fn hg_shock_state<'a>(&'a mut self) -> FieldHgShockState<'a> {
        FieldHgShockState(self)
    }
    pub fn hg_wu_change_ia<'a>(&'a mut self) -> FieldHgWuChangeIa<'a> {
        FieldHgWuChangeIa(self)
    }
    pub fn hg_wu_ia<'a>(&'a mut self) -> FieldHgWuIa<'a> {
        FieldHgWuIa(self)
    }
    pub fn hg_x_wu<'a>(&'a mut self) -> FieldHgXWu<'a> {
        FieldHgXWu(self)
    }
    pub fn hg_y_wu<'a>(&'a mut self) -> FieldHgYWu<'a> {
        FieldHgYWu(self)
    }
    pub fn hg_z_wu<'a>(&'a mut self) -> FieldHgZWu<'a> {
        FieldHgZWu(self)
    }
}
pub struct FieldHgShockChangeIa<'a>(pub &'a mut HgWakeUpSrcVal);
impl<'a> FieldHgShockChangeIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldHgShockState<'a>(pub &'a mut HgWakeUpSrcVal);
impl<'a> FieldHgShockState<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldHgWuChangeIa<'a>(pub &'a mut HgWakeUpSrcVal);
impl<'a> FieldHgWuChangeIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldHgWuIa<'a>(pub &'a mut HgWakeUpSrcVal);
impl<'a> FieldHgWuIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldHgXWu<'a>(pub &'a mut HgWakeUpSrcVal);
impl<'a> FieldHgXWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldHgYWu<'a>(pub &'a mut HgWakeUpSrcVal);
impl<'a> FieldHgYWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldHgZWu<'a>(pub &'a mut HgWakeUpSrcVal);
impl<'a> FieldHgZWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut HgWakeUpSrcVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut HgWakeUpSrcVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
