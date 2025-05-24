use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncFifoEnA<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncFifoEnA<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncFifoEnAVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x44, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncFifoEnAVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncFifoEnAVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x44, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncFifoEnAVal(val))
    }
    pub fn write(&mut self, val: EmbFuncFifoEnAVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x44, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncFifoEnAVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncFifoEnAVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x44, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncFifoEnAVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncFifoEnAVal) -> EmbFuncFifoEnAVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncFifoEnAVal) -> EmbFuncFifoEnAVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncFifoEnAVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncFifoEnAVal(0x0)).await
    }
}
pub struct EmbFuncFifoEnAVal(pub u8);
impl EmbFuncFifoEnAVal {
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
    pub fn mlc_fifo_en<'a>(&'a mut self) -> FieldMlcFifoEn<'a> {
        FieldMlcFifoEn(self)
    }
    pub fn step_counter_fifo_en<'a>(&'a mut self) -> FieldStepCounterFifoEn<'a> {
        FieldStepCounterFifoEn(self)
    }
    pub fn sflp_gbias_fifo_en<'a>(&'a mut self) -> FieldSflpGbiasFifoEn<'a> {
        FieldSflpGbiasFifoEn(self)
    }
    pub fn sflp_gravity_fifo_en<'a>(&'a mut self) -> FieldSflpGravityFifoEn<'a> {
        FieldSflpGravityFifoEn(self)
    }
    pub fn sflp_game_fifo_en<'a>(&'a mut self) -> FieldSflpGameFifoEn<'a> {
        FieldSflpGameFifoEn(self)
    }
}
pub struct FieldMlcFifoEn<'a>(pub &'a mut EmbFuncFifoEnAVal);
impl<'a> FieldMlcFifoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldStepCounterFifoEn<'a>(pub &'a mut EmbFuncFifoEnAVal);
impl<'a> FieldStepCounterFifoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldSflpGbiasFifoEn<'a>(pub &'a mut EmbFuncFifoEnAVal);
impl<'a> FieldSflpGbiasFifoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldSflpGravityFifoEn<'a>(pub &'a mut EmbFuncFifoEnAVal);
impl<'a> FieldSflpGravityFifoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldSflpGameFifoEn<'a>(pub &'a mut EmbFuncFifoEnAVal);
impl<'a> FieldSflpGameFifoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncFifoEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncFifoEnAVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
