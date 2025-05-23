use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Int1Ctrl<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Int1Ctrl<'a, C> {
    pub fn read(&mut self) -> Result<Int1CtrlVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0xd, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Int1CtrlVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Int1CtrlVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0xd, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Int1CtrlVal(val))
    }
    pub fn write(&mut self, val: Int1CtrlVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0xd, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Int1CtrlVal(raw_val))
    }
    pub async fn write_async(&mut self, val: Int1CtrlVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0xd, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Int1CtrlVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(Int1CtrlVal) -> Int1CtrlVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Int1CtrlVal) -> Int1CtrlVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Int1CtrlVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Int1CtrlVal(0x0)).await
    }
}
pub struct Int1CtrlVal(pub u8);
impl Int1CtrlVal {
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
    pub fn int1_cnt_bdr<'a>(&'a mut self) -> FieldInt1CntBdr<'a> {
        FieldInt1CntBdr(self)
    }
    pub fn int1_fifo_full<'a>(&'a mut self) -> FieldInt1FifoFull<'a> {
        FieldInt1FifoFull(self)
    }
    pub fn int1_fifo_ovr<'a>(&'a mut self) -> FieldInt1FifoOvr<'a> {
        FieldInt1FifoOvr(self)
    }
    pub fn int1_fifo_th<'a>(&'a mut self) -> FieldInt1FifoTh<'a> {
        FieldInt1FifoTh(self)
    }
    pub fn int1_drdy_g<'a>(&'a mut self) -> FieldInt1DrdyG<'a> {
        FieldInt1DrdyG(self)
    }
    pub fn int1_drdy_xl<'a>(&'a mut self) -> FieldInt1DrdyXl<'a> {
        FieldInt1DrdyXl(self)
    }
}
pub struct FieldInt1CntBdr<'a>(pub &'a mut Int1CtrlVal);
impl<'a> FieldInt1CntBdr<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldInt1FifoFull<'a>(pub &'a mut Int1CtrlVal);
impl<'a> FieldInt1FifoFull<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt1FifoOvr<'a>(pub &'a mut Int1CtrlVal);
impl<'a> FieldInt1FifoOvr<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldInt1FifoTh<'a>(pub &'a mut Int1CtrlVal);
impl<'a> FieldInt1FifoTh<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldInt1DrdyG<'a>(pub &'a mut Int1CtrlVal);
impl<'a> FieldInt1DrdyG<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldInt1DrdyXl<'a>(pub &'a mut Int1CtrlVal);
impl<'a> FieldInt1DrdyXl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Int1CtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Int1CtrlVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
