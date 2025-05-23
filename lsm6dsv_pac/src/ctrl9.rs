use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl9<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl9<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl9Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x18, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl9Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl9Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x18, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl9Val(val))
    }
    pub fn write(&mut self, val: Ctrl9Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x18, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl9Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl9Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x18, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl9Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl9Val) -> Ctrl9Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl9Val) -> Ctrl9Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl9Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl9Val(0x0)).await
    }
}
pub struct Ctrl9Val(pub u8);
impl Ctrl9Val {
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
    pub fn hp_ref_mode_xl<'a>(&'a mut self) -> FieldHpRefModeXl<'a> {
        FieldHpRefModeXl(self)
    }
    pub fn xl_fastsettl_mode<'a>(&'a mut self) -> FieldXlFastsettlMode<'a> {
        FieldXlFastsettlMode(self)
    }
    pub fn hp_slope_xl_en<'a>(&'a mut self) -> FieldHpSlopeXlEn<'a> {
        FieldHpSlopeXlEn(self)
    }
    pub fn lpf2_xl_en<'a>(&'a mut self) -> FieldLpf2XlEn<'a> {
        FieldLpf2XlEn(self)
    }
    pub fn usr_off_w<'a>(&'a mut self) -> FieldUsrOffW<'a> {
        FieldUsrOffW(self)
    }
    pub fn usr_off_on_out<'a>(&'a mut self) -> FieldUsrOffOnOut<'a> {
        FieldUsrOffOnOut(self)
    }
}
pub struct FieldHpRefModeXl<'a>(pub &'a mut Ctrl9Val);
impl<'a> FieldHpRefModeXl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldXlFastsettlMode<'a>(pub &'a mut Ctrl9Val);
impl<'a> FieldXlFastsettlMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldHpSlopeXlEn<'a>(pub &'a mut Ctrl9Val);
impl<'a> FieldHpSlopeXlEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldLpf2XlEn<'a>(pub &'a mut Ctrl9Val);
impl<'a> FieldLpf2XlEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldUsrOffW<'a>(pub &'a mut Ctrl9Val);
impl<'a> FieldUsrOffW<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldUsrOffOnOut<'a>(pub &'a mut Ctrl9Val);
impl<'a> FieldUsrOffOnOut<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl9Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
