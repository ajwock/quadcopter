use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl2XlHg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl2XlHg<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl2XlHgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x4d, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl2XlHgVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl2XlHgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x4d, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl2XlHgVal(val))
    }
    pub fn write(&mut self, val: Ctrl2XlHgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x4d, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl2XlHgVal(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl2XlHgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x4d, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl2XlHgVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl2XlHgVal) -> Ctrl2XlHgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl2XlHgVal) -> Ctrl2XlHgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl2XlHgVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl2XlHgVal(0x0)).await
    }
}
pub struct Ctrl2XlHgVal(pub u8);
impl Ctrl2XlHgVal {
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
    pub fn hg_usr_off_on_wu<'a>(&'a mut self) -> FieldHgUsrOffOnWu<'a> {
        FieldHgUsrOffOnWu(self)
    }
    pub fn xl_hg_st<'a>(&'a mut self) -> FieldXlHgSt<'a> {
        FieldXlHgSt(self)
    }
}
pub struct FieldHgUsrOffOnWu<'a>(pub &'a mut Ctrl2XlHgVal);
impl<'a> FieldHgUsrOffOnWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl2XlHgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl2XlHgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl2XlHgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl2XlHgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldXlHgSt<'a>(pub &'a mut Ctrl2XlHgVal);
impl<'a> FieldXlHgSt<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl2XlHgVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl2XlHgVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x0 & (!(!0 << 2) << 0);
        self.0
    }
}
