use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncInt1<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncInt1<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncInt1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0xa, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncInt1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncInt1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0xa, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncInt1Val(val))
    }
    pub fn write(&mut self, val: EmbFuncInt1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0xa, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncInt1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncInt1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0xa, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncInt1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncInt1Val) -> EmbFuncInt1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncInt1Val) -> EmbFuncInt1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncInt1Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncInt1Val(0x0)).await
    }
}
pub struct EmbFuncInt1Val(pub u8);
impl EmbFuncInt1Val {
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
    pub fn int1_fsm_lc<'a>(&'a mut self) -> FieldInt1FsmLc<'a> {
        FieldInt1FsmLc(self)
    }
    pub fn int1_sig_mot<'a>(&'a mut self) -> FieldInt1SigMot<'a> {
        FieldInt1SigMot(self)
    }
    pub fn int1_tilt<'a>(&'a mut self) -> FieldInt1Tilt<'a> {
        FieldInt1Tilt(self)
    }
    pub fn int1_step_detector<'a>(&'a mut self) -> FieldInt1StepDetector<'a> {
        FieldInt1StepDetector(self)
    }
}
pub struct FieldInt1FsmLc<'a>(pub &'a mut EmbFuncInt1Val);
impl<'a> FieldInt1FsmLc<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInt1Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInt1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInt1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInt1Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldInt1SigMot<'a>(pub &'a mut EmbFuncInt1Val);
impl<'a> FieldInt1SigMot<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInt1Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInt1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInt1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInt1Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt1Tilt<'a>(pub &'a mut EmbFuncInt1Val);
impl<'a> FieldInt1Tilt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInt1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInt1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInt1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInt1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldInt1StepDetector<'a>(pub &'a mut EmbFuncInt1Val);
impl<'a> FieldInt1StepDetector<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInt1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInt1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInt1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInt1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
