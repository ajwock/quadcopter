use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct WakeUpThs<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> WakeUpThs<'a, C> {
    pub fn read(&mut self) -> Result<WakeUpThsVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x5b, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(WakeUpThsVal(val))
    }
    pub async fn read_async(&mut self) -> Result<WakeUpThsVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x5b, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(WakeUpThsVal(val))
    }
    pub fn write(&mut self, val: WakeUpThsVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x5b, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(WakeUpThsVal(raw_val))
    }
    pub async fn write_async(&mut self, val: WakeUpThsVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x5b, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(WakeUpThsVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(WakeUpThsVal) -> WakeUpThsVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(WakeUpThsVal) -> WakeUpThsVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(WakeUpThsVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(WakeUpThsVal(0x0)).await
    }
}
pub struct WakeUpThsVal(pub u8);
impl WakeUpThsVal {
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
    pub fn single_double_tap<'a>(&'a mut self) -> FieldSingleDoubleTap<'a> {
        FieldSingleDoubleTap(self)
    }
    pub fn usr_off_on_wu<'a>(&'a mut self) -> FieldUsrOffOnWu<'a> {
        FieldUsrOffOnWu(self)
    }
    pub fn wk_ths<'a>(&'a mut self) -> FieldWkThs<'a> {
        FieldWkThs(self)
    }
}
pub struct FieldSingleDoubleTap<'a>(pub &'a mut WakeUpThsVal);
impl<'a> FieldSingleDoubleTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WakeUpThsVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut WakeUpThsVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WakeUpThsVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut WakeUpThsVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldUsrOffOnWu<'a>(pub &'a mut WakeUpThsVal);
impl<'a> FieldUsrOffOnWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WakeUpThsVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut WakeUpThsVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WakeUpThsVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut WakeUpThsVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldWkThs<'a>(pub &'a mut WakeUpThsVal);
impl<'a> FieldWkThs<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 6)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut WakeUpThsVal {
        self.0.0 &= !(!(!0 << 6) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 6)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut WakeUpThsVal {
        self.0.0 &= !(!(!0 << 6) << 0);
        self.0.0 |= 0x0 & (!(!0 << 6) << 0);
        self.0
    }
}
