use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Md1Cfg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Md1Cfg<'a, C> {
    pub fn read(&mut self) -> Result<Md1CfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x5e, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Md1CfgVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Md1CfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x5e, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Md1CfgVal(val))
    }
    pub fn write(&mut self, val: Md1CfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x5e, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Md1CfgVal(raw_val))
    }
    pub async fn write_async(&mut self, val: Md1CfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x5e, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Md1CfgVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(Md1CfgVal) -> Md1CfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Md1CfgVal) -> Md1CfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Md1CfgVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Md1CfgVal(0x0)).await
    }
}
pub struct Md1CfgVal(pub u8);
impl Md1CfgVal {
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
    pub fn int1_sleep_change<'a>(&'a mut self) -> FieldInt1SleepChange<'a> {
        FieldInt1SleepChange(self)
    }
    pub fn int1_single_tap<'a>(&'a mut self) -> FieldInt1SingleTap<'a> {
        FieldInt1SingleTap(self)
    }
    pub fn int1_wu<'a>(&'a mut self) -> FieldInt1Wu<'a> {
        FieldInt1Wu(self)
    }
    pub fn int1_ff<'a>(&'a mut self) -> FieldInt1Ff<'a> {
        FieldInt1Ff(self)
    }
    pub fn int1_double_tap<'a>(&'a mut self) -> FieldInt1DoubleTap<'a> {
        FieldInt1DoubleTap(self)
    }
    pub fn int1_6_d<'a>(&'a mut self) -> FieldInt16D<'a> {
        FieldInt16D(self)
    }
    pub fn int1_emb_func<'a>(&'a mut self) -> FieldInt1EmbFunc<'a> {
        FieldInt1EmbFunc(self)
    }
    pub fn int1_shub<'a>(&'a mut self) -> FieldInt1Shub<'a> {
        FieldInt1Shub(self)
    }
}
pub struct FieldInt1SleepChange<'a>(pub &'a mut Md1CfgVal);
impl<'a> FieldInt1SleepChange<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md1CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md1CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldInt1SingleTap<'a>(pub &'a mut Md1CfgVal);
impl<'a> FieldInt1SingleTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md1CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md1CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldInt1Wu<'a>(pub &'a mut Md1CfgVal);
impl<'a> FieldInt1Wu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md1CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md1CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt1Ff<'a>(pub &'a mut Md1CfgVal);
impl<'a> FieldInt1Ff<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md1CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md1CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldInt1DoubleTap<'a>(pub &'a mut Md1CfgVal);
impl<'a> FieldInt1DoubleTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md1CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md1CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldInt16D<'a>(pub &'a mut Md1CfgVal);
impl<'a> FieldInt16D<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md1CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md1CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldInt1EmbFunc<'a>(pub &'a mut Md1CfgVal);
impl<'a> FieldInt1EmbFunc<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md1CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md1CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldInt1Shub<'a>(pub &'a mut Md1CfgVal);
impl<'a> FieldInt1Shub<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md1CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md1CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md1CfgVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
