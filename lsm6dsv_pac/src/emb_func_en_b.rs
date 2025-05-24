use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncEnB<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncEnB<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncEnBVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x5, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncEnBVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncEnBVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x5, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncEnBVal(val))
    }
    pub fn write(&mut self, val: EmbFuncEnBVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x5, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncEnBVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncEnBVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x5, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncEnBVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncEnBVal) -> EmbFuncEnBVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncEnBVal) -> EmbFuncEnBVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncEnBVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncEnBVal(0x0)).await
    }
}
pub struct EmbFuncEnBVal(pub u8);
impl EmbFuncEnBVal {
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
    pub fn mlc_en<'a>(&'a mut self) -> FieldMlcEn<'a> {
        FieldMlcEn(self)
    }
    pub fn fifo_compr_en<'a>(&'a mut self) -> FieldFifoComprEn<'a> {
        FieldFifoComprEn(self)
    }
    pub fn fsm_en<'a>(&'a mut self) -> FieldFsmEn<'a> {
        FieldFsmEn(self)
    }
}
pub struct FieldMlcEn<'a>(pub &'a mut EmbFuncEnBVal);
impl<'a> FieldMlcEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncEnBVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncEnBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncEnBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncEnBVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldFifoComprEn<'a>(pub &'a mut EmbFuncEnBVal);
impl<'a> FieldFifoComprEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncEnBVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncEnBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncEnBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncEnBVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldFsmEn<'a>(pub &'a mut EmbFuncEnBVal);
impl<'a> FieldFsmEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncEnBVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncEnBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncEnBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncEnBVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
