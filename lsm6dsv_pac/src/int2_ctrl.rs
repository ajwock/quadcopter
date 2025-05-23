use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Int2Ctrl<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Int2Ctrl<'a, C> {
    pub fn read(&mut self) -> Result<Int2CtrlVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0xe, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Int2CtrlVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Int2CtrlVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0xe, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Int2CtrlVal(val))
    }
    pub fn write(&mut self, val: Int2CtrlVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0xe, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Int2CtrlVal(raw_val))
    }
    pub async fn write_async(&mut self, val: Int2CtrlVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0xe, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Int2CtrlVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(Int2CtrlVal) -> Int2CtrlVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Int2CtrlVal) -> Int2CtrlVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Int2CtrlVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Int2CtrlVal(0x0)).await
    }
}
pub struct Int2CtrlVal(pub u8);
impl Int2CtrlVal {
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
    pub fn int2_emb_func_endop<'a>(&'a mut self) -> FieldInt2EmbFuncEndop<'a> {
        FieldInt2EmbFuncEndop(self)
    }
    pub fn int2_cnt_bdr<'a>(&'a mut self) -> FieldInt2CntBdr<'a> {
        FieldInt2CntBdr(self)
    }
    pub fn int2_fifo_full<'a>(&'a mut self) -> FieldInt2FifoFull<'a> {
        FieldInt2FifoFull(self)
    }
    pub fn int2_fifo_ovr<'a>(&'a mut self) -> FieldInt2FifoOvr<'a> {
        FieldInt2FifoOvr(self)
    }
    pub fn int2_fifo_th<'a>(&'a mut self) -> FieldInt2FifoTh<'a> {
        FieldInt2FifoTh(self)
    }
    pub fn int2_drdy_g<'a>(&'a mut self) -> FieldInt2DrdyG<'a> {
        FieldInt2DrdyG(self)
    }
    pub fn int2_drdy_xl<'a>(&'a mut self) -> FieldInt2DrdyXl<'a> {
        FieldInt2DrdyXl(self)
    }
}
pub struct FieldInt2EmbFuncEndop<'a>(pub &'a mut Int2CtrlVal);
impl<'a> FieldInt2EmbFuncEndop<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldInt2CntBdr<'a>(pub &'a mut Int2CtrlVal);
impl<'a> FieldInt2CntBdr<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldInt2FifoFull<'a>(pub &'a mut Int2CtrlVal);
impl<'a> FieldInt2FifoFull<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt2FifoOvr<'a>(pub &'a mut Int2CtrlVal);
impl<'a> FieldInt2FifoOvr<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldInt2FifoTh<'a>(pub &'a mut Int2CtrlVal);
impl<'a> FieldInt2FifoTh<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldInt2DrdyG<'a>(pub &'a mut Int2CtrlVal);
impl<'a> FieldInt2DrdyG<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldInt2DrdyXl<'a>(pub &'a mut Int2CtrlVal);
impl<'a> FieldInt2DrdyXl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int2CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int2CtrlVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
