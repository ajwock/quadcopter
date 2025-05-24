use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncStatus<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncStatus<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x12, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncStatusVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x12, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncStatusVal(val))
    }
}
pub struct EmbFuncStatusVal(pub u8);
impl EmbFuncStatusVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn is_fsm_lc<'a>(&'a mut self) -> FieldIsFsmLc<'a> {
        FieldIsFsmLc(self)
    }
    pub fn is_sigmot<'a>(&'a mut self) -> FieldIsSigmot<'a> {
        FieldIsSigmot(self)
    }
    pub fn is_tilt<'a>(&'a mut self) -> FieldIsTilt<'a> {
        FieldIsTilt(self)
    }
    pub fn is_step_det<'a>(&'a mut self) -> FieldIsStepDet<'a> {
        FieldIsStepDet(self)
    }
}
pub struct FieldIsFsmLc<'a>(pub &'a mut EmbFuncStatusVal);
impl<'a> FieldIsFsmLc<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsSigmot<'a>(pub &'a mut EmbFuncStatusVal);
impl<'a> FieldIsSigmot<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsTilt<'a>(pub &'a mut EmbFuncStatusVal);
impl<'a> FieldIsTilt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsStepDet<'a>(pub &'a mut EmbFuncStatusVal);
impl<'a> FieldIsStepDet<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
