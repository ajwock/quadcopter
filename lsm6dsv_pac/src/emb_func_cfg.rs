use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncCfg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncCfg<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncCfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x63, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncCfgVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncCfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x63, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncCfgVal(val))
    }
    pub fn write(&mut self, val: EmbFuncCfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x63, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncCfgVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncCfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x63, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncCfgVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncCfgVal) -> EmbFuncCfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncCfgVal) -> EmbFuncCfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncCfgVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncCfgVal(0x0)).await
    }
}
pub struct EmbFuncCfgVal(pub u8);
impl EmbFuncCfgVal {
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
    pub fn hg_usr_off_on_emb_func<'a>(&'a mut self) -> FieldHgUsrOffOnEmbFunc<'a> {
        FieldHgUsrOffOnEmbFunc(self)
    }
    pub fn emb_func_irq_mask_xl_hg_settl<'a>(&'a mut self) -> FieldEmbFuncIrqMaskXlHgSettl<'a> {
        FieldEmbFuncIrqMaskXlHgSettl(self)
    }
    pub fn emb_func_irq_mask_g_settl<'a>(&'a mut self) -> FieldEmbFuncIrqMaskGSettl<'a> {
        FieldEmbFuncIrqMaskGSettl(self)
    }
    pub fn emb_func_irq_mask_xl_settl<'a>(&'a mut self) -> FieldEmbFuncIrqMaskXlSettl<'a> {
        FieldEmbFuncIrqMaskXlSettl(self)
    }
    pub fn emb_func_disable<'a>(&'a mut self) -> FieldEmbFuncDisable<'a> {
        FieldEmbFuncDisable(self)
    }
}
pub struct FieldHgUsrOffOnEmbFunc<'a>(pub &'a mut EmbFuncCfgVal);
impl<'a> FieldHgUsrOffOnEmbFunc<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldEmbFuncIrqMaskXlHgSettl<'a>(pub &'a mut EmbFuncCfgVal);
impl<'a> FieldEmbFuncIrqMaskXlHgSettl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldEmbFuncIrqMaskGSettl<'a>(pub &'a mut EmbFuncCfgVal);
impl<'a> FieldEmbFuncIrqMaskGSettl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldEmbFuncIrqMaskXlSettl<'a>(pub &'a mut EmbFuncCfgVal);
impl<'a> FieldEmbFuncIrqMaskXlSettl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldEmbFuncDisable<'a>(pub &'a mut EmbFuncCfgVal);
impl<'a> FieldEmbFuncDisable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncCfgVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
