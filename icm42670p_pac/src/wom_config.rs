use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct WomConfig<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> WomConfig<'a, C> {
    pub fn read(&mut self) -> Result<WomConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x27, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(WomConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<WomConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x27, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(WomConfigVal(val))
    }
    pub fn write(&mut self, val: WomConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x27, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(WomConfigVal(raw_val))
    }
    pub async fn write_async(&mut self, val: WomConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x27, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(WomConfigVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(WomConfigVal) -> WomConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(WomConfigVal) -> WomConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(WomConfigVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(WomConfigVal(0x0)).await
    }
}
pub struct WomConfigVal(pub u8);
impl WomConfigVal {
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
    pub fn wom_int_dur<'a>(&'a mut self) -> FieldWomIntDur<'a> {
        FieldWomIntDur(self)
    }
    pub fn wom_int_mode<'a>(&'a mut self) -> FieldWomIntMode<'a> {
        FieldWomIntMode(self)
    }
    pub fn wom_mode<'a>(&'a mut self) -> FieldWomMode<'a> {
        FieldWomMode(self)
    }
    pub fn wom_en<'a>(&'a mut self) -> FieldWomEn<'a> {
        FieldWomEn(self)
    }
}
pub struct FieldWomIntDur<'a>(pub &'a mut WomConfigVal);
impl<'a> FieldWomIntDur<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut WomConfigVal {
        self.0.0 &= !(!(!0 << 2) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut WomConfigVal {
        self.0.0 &= !(!(!0 << 2) << 3);
        self.0.0 |= 0x0 & (!(!0 << 2) << 3);
        self.0
    }
}
pub struct FieldWomIntMode<'a>(pub &'a mut WomConfigVal);
impl<'a> FieldWomIntMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut WomConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WomConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldWomMode<'a>(pub &'a mut WomConfigVal);
impl<'a> FieldWomMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut WomConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WomConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldWomEn<'a>(pub &'a mut WomConfigVal);
impl<'a> FieldWomEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut WomConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WomConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
