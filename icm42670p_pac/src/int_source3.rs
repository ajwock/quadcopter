use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntSource3<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> IntSource3<'a, D, C> {
    pub fn read(&mut self) -> Result<IntSource3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x2d, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x2d, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource3Val(val))
    }
    pub fn write(&mut self, val: IntSource3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x2d, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntSource3Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntSource3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x2d, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntSource3Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntSource3Val) -> IntSource3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntSource3Val) -> IntSource3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntSource3Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntSource3Val(0x0)).await
    }
}
pub struct IntSource3Val(pub u8);
impl IntSource3Val {
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
    pub fn st_int2_en<'a>(&'a mut self) -> FieldStInt2En<'a> {
        FieldStInt2En(self)
    }
    pub fn fsync_int2_en<'a>(&'a mut self) -> FieldFsyncInt2En<'a> {
        FieldFsyncInt2En(self)
    }
    pub fn pll_rdy_int2_en<'a>(&'a mut self) -> FieldPllRdyInt2En<'a> {
        FieldPllRdyInt2En(self)
    }
    pub fn reset_done_int2_en<'a>(&'a mut self) -> FieldResetDoneInt2En<'a> {
        FieldResetDoneInt2En(self)
    }
    pub fn drdy_int2_en<'a>(&'a mut self) -> FieldDrdyInt2En<'a> {
        FieldDrdyInt2En(self)
    }
    pub fn fifo_ths_int2_en<'a>(&'a mut self) -> FieldFifoThsInt2En<'a> {
        FieldFifoThsInt2En(self)
    }
    pub fn fifo_full_int2_en<'a>(&'a mut self) -> FieldFifoFullInt2En<'a> {
        FieldFifoFullInt2En(self)
    }
    pub fn agc_rdy_int2_en<'a>(&'a mut self) -> FieldAgcRdyInt2En<'a> {
        FieldAgcRdyInt2En(self)
    }
}
pub struct FieldStInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FieldStInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldFsyncInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FieldFsyncInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldPllRdyInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FieldPllRdyInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldResetDoneInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FieldResetDoneInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldDrdyInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FieldDrdyInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldFifoThsInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FieldFifoThsInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldFifoFullInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FieldFifoFullInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldAgcRdyInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FieldAgcRdyInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
