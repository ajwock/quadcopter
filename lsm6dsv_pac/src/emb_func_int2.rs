use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncInt2<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncInt2<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncInt2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0xe, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncInt2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncInt2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0xe, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncInt2Val(val))
    }
    pub fn write(&mut self, val: EmbFuncInt2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0xe, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncInt2Val(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncInt2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0xe, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncInt2Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncInt2Val) -> EmbFuncInt2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncInt2Val) -> EmbFuncInt2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncInt2Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncInt2Val(0x0)).await
    }
}
pub struct EmbFuncInt2Val(pub u8);
impl EmbFuncInt2Val {
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
    pub fn int2_fsm_lc<'a>(&'a mut self) -> FieldInt2FsmLc<'a> {
        FieldInt2FsmLc(self)
    }
    pub fn int_sig_mot<'a>(&'a mut self) -> FieldIntSigMot<'a> {
        FieldIntSigMot(self)
    }
    pub fn int2_tilt<'a>(&'a mut self) -> FieldInt2Tilt<'a> {
        FieldInt2Tilt(self)
    }
    pub fn int2_step_detector<'a>(&'a mut self) -> FieldInt2StepDetector<'a> {
        FieldInt2StepDetector(self)
    }
}
pub struct FieldInt2FsmLc<'a>(pub &'a mut EmbFuncInt2Val);
impl<'a> FieldInt2FsmLc<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInt2Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInt2Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldIntSigMot<'a>(pub &'a mut EmbFuncInt2Val);
impl<'a> FieldIntSigMot<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInt2Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInt2Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt2Tilt<'a>(pub &'a mut EmbFuncInt2Val);
impl<'a> FieldInt2Tilt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInt2Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInt2Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldInt2StepDetector<'a>(pub &'a mut EmbFuncInt2Val);
impl<'a> FieldInt2StepDetector<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInt2Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInt2Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
