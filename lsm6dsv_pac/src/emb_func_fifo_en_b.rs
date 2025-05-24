use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncFifoEnB<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncFifoEnB<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncFifoEnBVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x45, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncFifoEnBVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncFifoEnBVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x45, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncFifoEnBVal(val))
    }
    pub fn write(&mut self, val: EmbFuncFifoEnBVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x45, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncFifoEnBVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncFifoEnBVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x45, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncFifoEnBVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncFifoEnBVal) -> EmbFuncFifoEnBVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncFifoEnBVal) -> EmbFuncFifoEnBVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncFifoEnBVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncFifoEnBVal(0x0)).await
    }
}
pub struct EmbFuncFifoEnBVal(pub u8);
impl EmbFuncFifoEnBVal {
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
    pub fn fsm_fifo_en<'a>(&'a mut self) -> FieldFsmFifoEn<'a> {
        FieldFsmFifoEn(self)
    }
    pub fn mlc_filter_feature_fifo_en<'a>(&'a mut self) -> FieldMlcFilterFeatureFifoEn<'a> {
        FieldMlcFilterFeatureFifoEn(self)
    }
}
pub struct FieldFsmFifoEn<'a>(pub &'a mut EmbFuncFifoEnBVal);
impl<'a> FieldFsmFifoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncFifoEnBVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncFifoEnBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncFifoEnBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncFifoEnBVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldMlcFilterFeatureFifoEn<'a>(pub &'a mut EmbFuncFifoEnBVal);
impl<'a> FieldMlcFilterFeatureFifoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncFifoEnBVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncFifoEnBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncFifoEnBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncFifoEnBVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
