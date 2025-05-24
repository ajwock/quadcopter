use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncInitB<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncInitB<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncInitBVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x67, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncInitBVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncInitBVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x67, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncInitBVal(val))
    }
    pub fn write(&mut self, val: EmbFuncInitBVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x67, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncInitBVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncInitBVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x67, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncInitBVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncInitBVal) -> EmbFuncInitBVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncInitBVal) -> EmbFuncInitBVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncInitBVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncInitBVal(0x0)).await
    }
}
pub struct EmbFuncInitBVal(pub u8);
impl EmbFuncInitBVal {
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
    pub fn mlc_init<'a>(&'a mut self) -> FieldMlcInit<'a> {
        FieldMlcInit(self)
    }
    pub fn fifo_compr_init<'a>(&'a mut self) -> FieldFifoComprInit<'a> {
        FieldFifoComprInit(self)
    }
    pub fn pt_init<'a>(&'a mut self) -> FieldPtInit<'a> {
        FieldPtInit(self)
    }
    pub fn fsm_init<'a>(&'a mut self) -> FieldFsmInit<'a> {
        FieldFsmInit(self)
    }
}
pub struct FieldMlcInit<'a>(pub &'a mut EmbFuncInitBVal);
impl<'a> FieldMlcInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitBVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitBVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldFifoComprInit<'a>(pub &'a mut EmbFuncInitBVal);
impl<'a> FieldFifoComprInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitBVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitBVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldPtInit<'a>(pub &'a mut EmbFuncInitBVal);
impl<'a> FieldPtInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitBVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitBVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldFsmInit<'a>(pub &'a mut EmbFuncInitBVal);
impl<'a> FieldFsmInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitBVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitBVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitBVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitBVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
