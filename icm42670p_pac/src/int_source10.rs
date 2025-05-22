use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntSource10<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource10<'a, C> {
    pub fn read(&mut self) -> Result<IntSource10Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x33, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource10Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource10Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x33, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource10Val(val))
    }
    pub fn write(&mut self, val: IntSource10Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x33, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntSource10Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntSource10Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x33, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntSource10Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntSource10Val) -> IntSource10Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntSource10Val) -> IntSource10Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntSource10Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntSource10Val(0x0)).await
    }
}
pub struct IntSource10Val(pub u8);
impl IntSource10Val {
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
    pub fn step_det_ibi_en<'a>(&'a mut self) -> FieldStepDetIbiEn<'a> {
        FieldStepDetIbiEn(self)
    }
    pub fn step_cnt_ofl_ibi_en<'a>(&'a mut self) -> FieldStepCntOflIbiEn<'a> {
        FieldStepCntOflIbiEn(self)
    }
    pub fn tilt_det_ibi_en<'a>(&'a mut self) -> FieldTiltDetIbiEn<'a> {
        FieldTiltDetIbiEn(self)
    }
}
pub struct FieldStepDetIbiEn<'a>(pub &'a mut IntSource10Val);
impl<'a> FieldStepDetIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource10Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource10Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource10Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource10Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldStepCntOflIbiEn<'a>(pub &'a mut IntSource10Val);
impl<'a> FieldStepCntOflIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource10Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource10Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource10Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource10Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldTiltDetIbiEn<'a>(pub &'a mut IntSource10Val);
impl<'a> FieldTiltDetIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource10Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource10Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource10Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource10Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
