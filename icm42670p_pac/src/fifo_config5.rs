use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct FifoConfig5<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> FifoConfig5<'a, D, C> {
    pub fn read(&mut self) -> Result<FifoConfig5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x1, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig5Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoConfig5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x1, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig5Val(val))
    }
    pub fn write(&mut self, val: FifoConfig5Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x1, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FifoConfig5Val(raw_val))
    }
    pub async fn write_async(&mut self, val: FifoConfig5Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x1, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FifoConfig5Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(FifoConfig5Val) -> FifoConfig5Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FifoConfig5Val) -> FifoConfig5Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FifoConfig5Val(0x20))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FifoConfig5Val(0x20)).await
    }
}
pub struct FifoConfig5Val(pub u8);
impl FifoConfig5Val {
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
        Self(0x20)
    }
    pub fn fifo_wm_gt_th<'a>(&'a mut self) -> FieldFifoWmGtTh<'a> {
        FieldFifoWmGtTh(self)
    }
    pub fn fifo_resume_partial_rd<'a>(&'a mut self) -> FieldFifoResumePartialRd<'a> {
        FieldFifoResumePartialRd(self)
    }
    pub fn fifo_hires_en<'a>(&'a mut self) -> FieldFifoHiresEn<'a> {
        FieldFifoHiresEn(self)
    }
    pub fn fifo_tmst_fsync_en<'a>(&'a mut self) -> FieldFifoTmstFsyncEn<'a> {
        FieldFifoTmstFsyncEn(self)
    }
    pub fn fifo_gyro_en<'a>(&'a mut self) -> FieldFifoGyroEn<'a> {
        FieldFifoGyroEn(self)
    }
    pub fn fifo_accel_en<'a>(&'a mut self) -> FieldFifoAccelEn<'a> {
        FieldFifoAccelEn(self)
    }
}
pub struct FieldFifoWmGtTh<'a>(pub &'a mut FifoConfig5Val);
impl<'a> FieldFifoWmGtTh<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x20;
        self.0
    }
}
pub struct FieldFifoResumePartialRd<'a>(pub &'a mut FifoConfig5Val);
impl<'a> FieldFifoResumePartialRd<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x20;
        self.0
    }
}
pub struct FieldFifoHiresEn<'a>(pub &'a mut FifoConfig5Val);
impl<'a> FieldFifoHiresEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x20;
        self.0
    }
}
pub struct FieldFifoTmstFsyncEn<'a>(pub &'a mut FifoConfig5Val);
impl<'a> FieldFifoTmstFsyncEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x20;
        self.0
    }
}
pub struct FieldFifoGyroEn<'a>(pub &'a mut FifoConfig5Val);
impl<'a> FieldFifoGyroEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x20;
        self.0
    }
}
pub struct FieldFifoAccelEn<'a>(pub &'a mut FifoConfig5Val);
impl<'a> FieldFifoAccelEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig5Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoConfig5Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x20;
        self.0
    }
}
