use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct TapThs6D<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> TapThs6D<'a, C> {
    pub fn read(&mut self) -> Result<TapThs6DVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x59, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(TapThs6DVal(val))
    }
    pub async fn read_async(&mut self) -> Result<TapThs6DVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x59, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(TapThs6DVal(val))
    }
    pub fn write(&mut self, val: TapThs6DVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x59, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(TapThs6DVal(raw_val))
    }
    pub async fn write_async(&mut self, val: TapThs6DVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x59, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(TapThs6DVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(TapThs6DVal) -> TapThs6DVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(TapThs6DVal) -> TapThs6DVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(TapThs6DVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(TapThs6DVal(0x0)).await
    }
}
pub struct TapThs6DVal(pub u8);
impl TapThs6DVal {
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
    pub fn d4_d_en<'a>(&'a mut self) -> FieldD4DEn<'a> {
        FieldD4DEn(self)
    }
    pub fn sixd_ths<'a>(&'a mut self) -> FieldSixdThs<'a> {
        FieldSixdThs(self)
    }
    pub fn tap_ths_z<'a>(&'a mut self) -> FieldTapThsZ<'a> {
        FieldTapThsZ(self)
    }
}
pub struct FieldD4DEn<'a>(pub &'a mut TapThs6DVal);
impl<'a> FieldD4DEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut TapThs6DVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut TapThs6DVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut TapThs6DVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut TapThs6DVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldSixdThs<'a>(pub &'a mut TapThs6DVal);
impl<'a> FieldSixdThs<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TapThs6DVal {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 5;
        self.0
    }
    pub fn reset(self) -> &'a mut TapThs6DVal {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= 0x0 & (!(!0 << 2) << 5);
        self.0
    }
}
pub struct FieldTapThsZ<'a>(pub &'a mut TapThs6DVal);
impl<'a> FieldTapThsZ<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 5)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TapThs6DVal {
        self.0.0 &= !(!(!0 << 5) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 5)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut TapThs6DVal {
        self.0.0 &= !(!(!0 << 5) << 0);
        self.0.0 |= 0x0 & (!(!0 << 5) << 0);
        self.0
    }
}
