use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FifoCtrl2<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FifoCtrl2<'a, C> {
    pub fn read(&mut self) -> Result<FifoCtrl2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x8, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoCtrl2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoCtrl2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x8, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoCtrl2Val(val))
    }
    pub fn write(&mut self, val: FifoCtrl2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x8, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FifoCtrl2Val(raw_val))
    }
    pub async fn write_async(&mut self, val: FifoCtrl2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x8, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FifoCtrl2Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(FifoCtrl2Val) -> FifoCtrl2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FifoCtrl2Val) -> FifoCtrl2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FifoCtrl2Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FifoCtrl2Val(0x0)).await
    }
}
pub struct FifoCtrl2Val(pub u8);
impl FifoCtrl2Val {
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
    pub fn stop_on_wtm<'a>(&'a mut self) -> FieldStopOnWtm<'a> {
        FieldStopOnWtm(self)
    }
    pub fn fifo_compr_rt_en<'a>(&'a mut self) -> FieldFifoComprRtEn<'a> {
        FieldFifoComprRtEn(self)
    }
    pub fn odr_chg_en<'a>(&'a mut self) -> FieldOdrChgEn<'a> {
        FieldOdrChgEn(self)
    }
    pub fn uncompr_rate<'a>(&'a mut self) -> FieldUncomprRate<'a> {
        FieldUncomprRate(self)
    }
}
pub struct FieldStopOnWtm<'a>(pub &'a mut FifoCtrl2Val);
impl<'a> FieldStopOnWtm<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoCtrl2Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoCtrl2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoCtrl2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoCtrl2Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldFifoComprRtEn<'a>(pub &'a mut FifoCtrl2Val);
impl<'a> FieldFifoComprRtEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoCtrl2Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoCtrl2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoCtrl2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoCtrl2Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldOdrChgEn<'a>(pub &'a mut FifoCtrl2Val);
impl<'a> FieldOdrChgEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoCtrl2Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoCtrl2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoCtrl2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoCtrl2Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldUncomprRate<'a>(pub &'a mut FifoCtrl2Val);
impl<'a> FieldUncomprRate<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 1) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoCtrl2Val {
        self.0.0 &= !(!(!0 << 2) << 1);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 1;
        self.0
    }
    pub fn reset(self) -> &'a mut FifoCtrl2Val {
        self.0.0 &= !(!(!0 << 2) << 1);
        self.0.0 |= 0x0 & (!(!0 << 2) << 1);
        self.0
    }
}
