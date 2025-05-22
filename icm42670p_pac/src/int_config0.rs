use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntConfig0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntConfig0<'a, C> {
    pub fn read(&mut self) -> Result<IntConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x4, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x4, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntConfig0Val(val))
    }
    pub fn write(&mut self, val: IntConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x4, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntConfig0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x4, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntConfig0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntConfig0Val) -> IntConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntConfig0Val) -> IntConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntConfig0Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntConfig0Val(0x0)).await
    }
}
pub struct IntConfig0Val(pub u8);
impl IntConfig0Val {
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
    pub fn ui_drdy_int_clear<'a>(&'a mut self) -> FieldUiDrdyIntClear<'a> {
        FieldUiDrdyIntClear(self)
    }
    pub fn fifo_ths_int_clear<'a>(&'a mut self) -> FieldFifoThsIntClear<'a> {
        FieldFifoThsIntClear(self)
    }
    pub fn fifo_full_int_clear<'a>(&'a mut self) -> FieldFifoFullIntClear<'a> {
        FieldFifoFullIntClear(self)
    }
}
pub struct FieldUiDrdyIntClear<'a>(pub &'a mut IntConfig0Val);
impl<'a> FieldUiDrdyIntClear<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut IntConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut IntConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 4);
        self.0.0 |= 0x0 & (!(!0 << 2) << 4);
        self.0
    }
}
pub struct FieldFifoThsIntClear<'a>(pub &'a mut IntConfig0Val);
impl<'a> FieldFifoThsIntClear<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut IntConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 2;
        self.0
    }
    pub fn reset(self) -> &'a mut IntConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= 0x0 & (!(!0 << 2) << 2);
        self.0
    }
}
pub struct FieldFifoFullIntClear<'a>(pub &'a mut IntConfig0Val);
impl<'a> FieldFifoFullIntClear<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut IntConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut IntConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x0 & (!(!0 << 2) << 0);
        self.0
    }
}
