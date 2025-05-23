use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl4<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl4<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x13, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl4Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x13, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl4Val(val))
    }
    pub fn write(&mut self, val: Ctrl4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x13, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl4Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x13, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl4Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl4Val) -> Ctrl4Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl4Val) -> Ctrl4Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl4Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl4Val(0x0)).await
    }
}
pub struct Ctrl4Val(pub u8);
impl Ctrl4Val {
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
    pub fn int2_on_int1<'a>(&'a mut self) -> FieldInt2OnInt1<'a> {
        FieldInt2OnInt1(self)
    }
    pub fn drdy_mask<'a>(&'a mut self) -> FieldDrdyMask<'a> {
        FieldDrdyMask(self)
    }
    pub fn int2_drdy_temp<'a>(&'a mut self) -> FieldInt2DrdyTemp<'a> {
        FieldInt2DrdyTemp(self)
    }
    pub fn drdy_pulsed<'a>(&'a mut self) -> FieldDrdyPulsed<'a> {
        FieldDrdyPulsed(self)
    }
    pub fn int2_in_lh<'a>(&'a mut self) -> FieldInt2InLh<'a> {
        FieldInt2InLh(self)
    }
}
pub struct FieldInt2OnInt1<'a>(pub &'a mut Ctrl4Val);
impl<'a> FieldInt2OnInt1<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldDrdyMask<'a>(pub &'a mut Ctrl4Val);
impl<'a> FieldDrdyMask<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldInt2DrdyTemp<'a>(pub &'a mut Ctrl4Val);
impl<'a> FieldInt2DrdyTemp<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldDrdyPulsed<'a>(pub &'a mut Ctrl4Val);
impl<'a> FieldDrdyPulsed<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldInt2InLh<'a>(pub &'a mut Ctrl4Val);
impl<'a> FieldInt2InLh<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl4Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
