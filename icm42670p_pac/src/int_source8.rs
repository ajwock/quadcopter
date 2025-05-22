use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntSource8<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> IntSource8<'a, D, C> {
    pub fn read(&mut self) -> Result<IntSource8Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x31, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource8Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource8Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x31, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource8Val(val))
    }
    pub fn write(&mut self, val: IntSource8Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x31, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntSource8Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntSource8Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x31, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntSource8Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntSource8Val) -> IntSource8Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntSource8Val) -> IntSource8Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntSource8Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntSource8Val(0x0)).await
    }
}
pub struct IntSource8Val(pub u8);
impl IntSource8Val {
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
    pub fn fsync_ibi_en<'a>(&'a mut self) -> FieldFsyncIbiEn<'a> {
        FieldFsyncIbiEn(self)
    }
    pub fn pll_rdy_ibi_en<'a>(&'a mut self) -> FieldPllRdyIbiEn<'a> {
        FieldPllRdyIbiEn(self)
    }
    pub fn ui_drdy_ibi_en<'a>(&'a mut self) -> FieldUiDrdyIbiEn<'a> {
        FieldUiDrdyIbiEn(self)
    }
    pub fn fifo_ths_ibi_en<'a>(&'a mut self) -> FieldFifoThsIbiEn<'a> {
        FieldFifoThsIbiEn(self)
    }
    pub fn fifo_full_ibi_en<'a>(&'a mut self) -> FieldFifoFullIbiEn<'a> {
        FieldFifoFullIbiEn(self)
    }
    pub fn agc_rdy_ibi_en<'a>(&'a mut self) -> FieldAgcRdyIbiEn<'a> {
        FieldAgcRdyIbiEn(self)
    }
}
pub struct FieldFsyncIbiEn<'a>(pub &'a mut IntSource8Val);
impl<'a> FieldFsyncIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource8Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource8Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldPllRdyIbiEn<'a>(pub &'a mut IntSource8Val);
impl<'a> FieldPllRdyIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource8Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource8Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldUiDrdyIbiEn<'a>(pub &'a mut IntSource8Val);
impl<'a> FieldUiDrdyIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource8Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource8Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldFifoThsIbiEn<'a>(pub &'a mut IntSource8Val);
impl<'a> FieldFifoThsIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource8Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource8Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldFifoFullIbiEn<'a>(pub &'a mut IntSource8Val);
impl<'a> FieldFifoFullIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource8Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource8Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldAgcRdyIbiEn<'a>(pub &'a mut IntSource8Val);
impl<'a> FieldAgcRdyIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource8Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource8Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource8Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
