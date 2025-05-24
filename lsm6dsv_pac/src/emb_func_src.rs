use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncSrc<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncSrc<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x64, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncSrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x64, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncSrcVal(val))
    }
    pub fn write(&mut self, val: EmbFuncSrcVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x64, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncSrcVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncSrcVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x64, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncSrcVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncSrcVal) -> EmbFuncSrcVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncSrcVal) -> EmbFuncSrcVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
}
pub struct EmbFuncSrcVal(pub u8);
impl EmbFuncSrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
        Self(0)
    }
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }
    pub fn pedo_rst_step<'a>(&'a mut self) -> FieldPedoRstStep<'a> {
        FieldPedoRstStep(self)
    }
    pub fn step_detected<'a>(&'a mut self) -> FieldStepDetected<'a> {
        FieldStepDetected(self)
    }
    pub fn step_count_delta_ia<'a>(&'a mut self) -> FieldStepCountDeltaIa<'a> {
        FieldStepCountDeltaIa(self)
    }
    pub fn step_overflow<'a>(&'a mut self) -> FieldStepOverflow<'a> {
        FieldStepOverflow(self)
    }
    pub fn stepcounter_bit_set<'a>(&'a mut self) -> FieldStepcounterBitSet<'a> {
        FieldStepcounterBitSet(self)
    }
}
pub struct FieldPedoRstStep<'a>(pub &'a mut EmbFuncSrcVal);
impl<'a> FieldPedoRstStep<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSrcVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(false)
    }
}
pub struct FieldStepDetected<'a>(pub &'a mut EmbFuncSrcVal);
impl<'a> FieldStepDetected<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSrcVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(false)
    }
}
pub struct FieldStepCountDeltaIa<'a>(pub &'a mut EmbFuncSrcVal);
impl<'a> FieldStepCountDeltaIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSrcVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(false)
    }
}
pub struct FieldStepOverflow<'a>(pub &'a mut EmbFuncSrcVal);
impl<'a> FieldStepOverflow<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSrcVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(false)
    }
}
pub struct FieldStepcounterBitSet<'a>(pub &'a mut EmbFuncSrcVal);
impl<'a> FieldStepcounterBitSet<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSrcVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSrcVal {
        self.assign(false)
    }
}
