use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FuncCfgAccess<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FuncCfgAccess<'a, C> {
    pub fn read(&mut self) -> Result<FuncCfgAccessVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FuncCfgAccessVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FuncCfgAccessVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FuncCfgAccessVal(val))
    }
    pub fn write(&mut self, val: FuncCfgAccessVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x1, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FuncCfgAccessVal(raw_val))
    }
    pub async fn write_async(&mut self, val: FuncCfgAccessVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x1, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FuncCfgAccessVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(FuncCfgAccessVal) -> FuncCfgAccessVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FuncCfgAccessVal) -> FuncCfgAccessVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FuncCfgAccessVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FuncCfgAccessVal(0x0)).await
    }
}
pub struct FuncCfgAccessVal(pub u8);
impl FuncCfgAccessVal {
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
    pub fn emb_func_reg_access<'a>(&'a mut self) -> FieldEmbFuncRegAccess<'a> {
        FieldEmbFuncRegAccess(self)
    }
    pub fn shub_reg_access<'a>(&'a mut self) -> FieldShubRegAccess<'a> {
        FieldShubRegAccess(self)
    }
    pub fn fsm_wr_ctrl_en<'a>(&'a mut self) -> FieldFsmWrCtrlEn<'a> {
        FieldFsmWrCtrlEn(self)
    }
    pub fn sw_por<'a>(&'a mut self) -> FieldSwPor<'a> {
        FieldSwPor(self)
    }
}
pub struct FieldEmbFuncRegAccess<'a>(pub &'a mut FuncCfgAccessVal);
impl<'a> FieldEmbFuncRegAccess<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldShubRegAccess<'a>(pub &'a mut FuncCfgAccessVal);
impl<'a> FieldShubRegAccess<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldFsmWrCtrlEn<'a>(pub &'a mut FuncCfgAccessVal);
impl<'a> FieldFsmWrCtrlEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldSwPor<'a>(pub &'a mut FuncCfgAccessVal);
impl<'a> FieldSwPor<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
