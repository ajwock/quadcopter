use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct TapCfg0<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> TapCfg0<'a, C> {
    pub fn read(&mut self) -> Result<TapCfg0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x56, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(TapCfg0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<TapCfg0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x56, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(TapCfg0Val(val))
    }
    pub fn write(&mut self, val: TapCfg0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x56, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(TapCfg0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: TapCfg0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x56, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(TapCfg0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(TapCfg0Val) -> TapCfg0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(TapCfg0Val) -> TapCfg0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(TapCfg0Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(TapCfg0Val(0x0)).await
    }
}
pub struct TapCfg0Val(pub u8);
impl TapCfg0Val {
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
    pub fn low_pass_on_6_d<'a>(&'a mut self) -> FieldLowPassOn6D<'a> {
        FieldLowPassOn6D(self)
    }
    pub fn hw_func_mask_xl_settl<'a>(&'a mut self) -> FieldHwFuncMaskXlSettl<'a> {
        FieldHwFuncMaskXlSettl(self)
    }
    pub fn slope_fds<'a>(&'a mut self) -> FieldSlopeFds<'a> {
        FieldSlopeFds(self)
    }
    pub fn tap_x_en<'a>(&'a mut self) -> FieldTapXEn<'a> {
        FieldTapXEn(self)
    }
    pub fn tap_y_en<'a>(&'a mut self) -> FieldTapYEn<'a> {
        FieldTapYEn(self)
    }
    pub fn tap_z_en<'a>(&'a mut self) -> FieldTapZEn<'a> {
        FieldTapZEn(self)
    }
    pub fn lir<'a>(&'a mut self) -> FieldLir<'a> {
        FieldLir(self)
    }
}
pub struct FieldLowPassOn6D<'a>(pub &'a mut TapCfg0Val);
impl<'a> FieldLowPassOn6D<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut TapCfg0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut TapCfg0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldHwFuncMaskXlSettl<'a>(pub &'a mut TapCfg0Val);
impl<'a> FieldHwFuncMaskXlSettl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut TapCfg0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut TapCfg0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldSlopeFds<'a>(pub &'a mut TapCfg0Val);
impl<'a> FieldSlopeFds<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut TapCfg0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut TapCfg0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldTapXEn<'a>(pub &'a mut TapCfg0Val);
impl<'a> FieldTapXEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut TapCfg0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut TapCfg0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldTapYEn<'a>(pub &'a mut TapCfg0Val);
impl<'a> FieldTapYEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut TapCfg0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut TapCfg0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldTapZEn<'a>(pub &'a mut TapCfg0Val);
impl<'a> FieldTapZEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut TapCfg0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut TapCfg0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldLir<'a>(pub &'a mut TapCfg0Val);
impl<'a> FieldLir<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut TapCfg0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut TapCfg0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut TapCfg0Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
