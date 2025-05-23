use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FifoCtrl4<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FifoCtrl4<'a, C> {
    pub fn read(&mut self) -> Result<FifoCtrl4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0xa, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoCtrl4Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoCtrl4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0xa, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoCtrl4Val(val))
    }
    pub fn write(&mut self, val: FifoCtrl4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0xa, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FifoCtrl4Val(raw_val))
    }
    pub async fn write_async(&mut self, val: FifoCtrl4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0xa, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FifoCtrl4Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(FifoCtrl4Val) -> FifoCtrl4Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FifoCtrl4Val) -> FifoCtrl4Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FifoCtrl4Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FifoCtrl4Val(0x0)).await
    }
}
pub struct FifoCtrl4Val(pub u8);
impl FifoCtrl4Val {
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
    pub fn dec_ts_batch<'a>(&'a mut self) -> FieldDecTsBatch<'a> {
        FieldDecTsBatch(self)
    }
    pub fn odr_t_batch<'a>(&'a mut self) -> FieldOdrTBatch<'a> {
        FieldOdrTBatch(self)
    }
    pub fn fifo_mode<'a>(&'a mut self) -> FieldFifoMode<'a> {
        FieldFifoMode(self)
    }
}
pub struct FieldDecTsBatch<'a>(pub &'a mut FifoCtrl4Val);
impl<'a> FieldDecTsBatch<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 6) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoCtrl4Val {
        self.0.0 &= !(!(!0 << 2) << 6);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 6;
        self.0
    }
    pub fn reset(self) -> &'a mut FifoCtrl4Val {
        self.0.0 &= !(!(!0 << 2) << 6);
        self.0.0 |= 0x0 & (!(!0 << 2) << 6);
        self.0
    }
}
pub struct FieldOdrTBatch<'a>(pub &'a mut FifoCtrl4Val);
impl<'a> FieldOdrTBatch<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoCtrl4Val {
        self.0.0 &= !(!(!0 << 2) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut FifoCtrl4Val {
        self.0.0 &= !(!(!0 << 2) << 4);
        self.0.0 |= 0x0 & (!(!0 << 2) << 4);
        self.0
    }
}
pub struct FieldFifoMode<'a>(pub &'a mut FifoCtrl4Val);
impl<'a> FieldFifoMode<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoCtrl4Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut FifoCtrl4Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x0 & (!(!0 << 3) << 0);
        self.0
    }
}
