use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntSource7<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource7<'a, C> {
    pub fn read(&mut self) -> Result<IntSource7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x2f, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource7Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x2f, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource7Val(val))
    }
    pub fn write(&mut self, val: IntSource7Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x2f, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntSource7Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntSource7Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x2f, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntSource7Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntSource7Val) -> IntSource7Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntSource7Val) -> IntSource7Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntSource7Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntSource7Val(0x0)).await
    }
}
pub struct IntSource7Val(pub u8);
impl IntSource7Val {
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
    pub fn ff_int2_en<'a>(&'a mut self) -> FieldFfInt2En<'a> {
        FieldFfInt2En(self)
    }
    pub fn lowg_int2_en<'a>(&'a mut self) -> FieldLowgInt2En<'a> {
        FieldLowgInt2En(self)
    }
    pub fn step_det_int2_en<'a>(&'a mut self) -> FieldStepDetInt2En<'a> {
        FieldStepDetInt2En(self)
    }
    pub fn step_cnt_ofl_int2_en<'a>(&'a mut self) -> FieldStepCntOflInt2En<'a> {
        FieldStepCntOflInt2En(self)
    }
    pub fn tilt_det_int2_en<'a>(&'a mut self) -> FieldTiltDetInt2En<'a> {
        FieldTiltDetInt2En(self)
    }
}
pub struct FieldFfInt2En<'a>(pub &'a mut IntSource7Val);
impl<'a> FieldFfInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldLowgInt2En<'a>(pub &'a mut IntSource7Val);
impl<'a> FieldLowgInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldStepDetInt2En<'a>(pub &'a mut IntSource7Val);
impl<'a> FieldStepDetInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldStepCntOflInt2En<'a>(pub &'a mut IntSource7Val);
impl<'a> FieldStepCntOflInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldTiltDetInt2En<'a>(pub &'a mut IntSource7Val);
impl<'a> FieldTiltDetInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource7Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
