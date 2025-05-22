use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntSource0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource0<'a, C> {
    pub fn read(&mut self) -> Result<IntSource0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x2b, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x2b, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource0Val(val))
    }
    pub fn write(&mut self, val: IntSource0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x2b, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntSource0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntSource0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x2b, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntSource0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntSource0Val) -> IntSource0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntSource0Val) -> IntSource0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntSource0Val(0x10))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntSource0Val(0x10)).await
    }
}
pub struct IntSource0Val(pub u8);
impl IntSource0Val {
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
        Self(0x10)
    }
    pub fn st_int1_en<'a>(&'a mut self) -> FieldStInt1En<'a> {
        FieldStInt1En(self)
    }
    pub fn fsync_int1_en<'a>(&'a mut self) -> FieldFsyncInt1En<'a> {
        FieldFsyncInt1En(self)
    }
    pub fn pll_rdy_int1_en<'a>(&'a mut self) -> FieldPllRdyInt1En<'a> {
        FieldPllRdyInt1En(self)
    }
    pub fn reset_done_int1_en<'a>(&'a mut self) -> FieldResetDoneInt1En<'a> {
        FieldResetDoneInt1En(self)
    }
    pub fn drdy_int1_en<'a>(&'a mut self) -> FieldDrdyInt1En<'a> {
        FieldDrdyInt1En(self)
    }
    pub fn fifo_ths_int1_en<'a>(&'a mut self) -> FieldFifoThsInt1En<'a> {
        FieldFifoThsInt1En(self)
    }
    pub fn fifo_full_int1_en<'a>(&'a mut self) -> FieldFifoFullInt1En<'a> {
        FieldFifoFullInt1En(self)
    }
    pub fn agc_rdy_int1_en<'a>(&'a mut self) -> FieldAgcRdyInt1En<'a> {
        FieldAgcRdyInt1En(self)
    }
}
pub struct FieldStInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FieldStInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x10;
        self.0
    }
}
pub struct FieldFsyncInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FieldFsyncInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x10;
        self.0
    }
}
pub struct FieldPllRdyInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FieldPllRdyInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x10;
        self.0
    }
}
pub struct FieldResetDoneInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FieldResetDoneInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x10;
        self.0
    }
}
pub struct FieldDrdyInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FieldDrdyInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x10;
        self.0
    }
}
pub struct FieldFifoThsInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FieldFifoThsInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x10;
        self.0
    }
}
pub struct FieldFifoFullInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FieldFifoFullInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x10;
        self.0
    }
}
pub struct FieldAgcRdyInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FieldAgcRdyInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x10;
        self.0
    }
}
