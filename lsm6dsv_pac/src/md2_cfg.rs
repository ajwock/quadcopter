use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Md2Cfg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Md2Cfg<'a, C> {
    pub fn read(&mut self) -> Result<Md2CfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x5f, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Md2CfgVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Md2CfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x5f, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Md2CfgVal(val))
    }
    pub fn write(&mut self, val: Md2CfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x5f, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Md2CfgVal(raw_val))
    }
    pub async fn write_async(&mut self, val: Md2CfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x5f, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Md2CfgVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(Md2CfgVal) -> Md2CfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Md2CfgVal) -> Md2CfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Md2CfgVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Md2CfgVal(0x0)).await
    }
}
pub struct Md2CfgVal(pub u8);
impl Md2CfgVal {
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
    pub fn int2_sleep_change<'a>(&'a mut self) -> FieldInt2SleepChange<'a> {
        FieldInt2SleepChange(self)
    }
    pub fn int2_single_tap<'a>(&'a mut self) -> FieldInt2SingleTap<'a> {
        FieldInt2SingleTap(self)
    }
    pub fn int2_wu<'a>(&'a mut self) -> FieldInt2Wu<'a> {
        FieldInt2Wu(self)
    }
    pub fn int2_ff<'a>(&'a mut self) -> FieldInt2Ff<'a> {
        FieldInt2Ff(self)
    }
    pub fn int2_double_tap<'a>(&'a mut self) -> FieldInt2DoubleTap<'a> {
        FieldInt2DoubleTap(self)
    }
    pub fn int2_6_d<'a>(&'a mut self) -> FieldInt26D<'a> {
        FieldInt26D(self)
    }
    pub fn int2_emb_func<'a>(&'a mut self) -> FieldInt2EmbFunc<'a> {
        FieldInt2EmbFunc(self)
    }
    pub fn int2_timestamp<'a>(&'a mut self) -> FieldInt2Timestamp<'a> {
        FieldInt2Timestamp(self)
    }
}
pub struct FieldInt2SleepChange<'a>(pub &'a mut Md2CfgVal);
impl<'a> FieldInt2SleepChange<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md2CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md2CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldInt2SingleTap<'a>(pub &'a mut Md2CfgVal);
impl<'a> FieldInt2SingleTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md2CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md2CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldInt2Wu<'a>(pub &'a mut Md2CfgVal);
impl<'a> FieldInt2Wu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md2CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md2CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt2Ff<'a>(pub &'a mut Md2CfgVal);
impl<'a> FieldInt2Ff<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md2CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md2CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldInt2DoubleTap<'a>(pub &'a mut Md2CfgVal);
impl<'a> FieldInt2DoubleTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md2CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md2CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldInt26D<'a>(pub &'a mut Md2CfgVal);
impl<'a> FieldInt26D<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md2CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md2CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldInt2EmbFunc<'a>(pub &'a mut Md2CfgVal);
impl<'a> FieldInt2EmbFunc<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md2CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md2CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldInt2Timestamp<'a>(pub &'a mut Md2CfgVal);
impl<'a> FieldInt2Timestamp<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Md2CfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Md2CfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Md2CfgVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
