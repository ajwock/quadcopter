use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl1XlHg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl1XlHg<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl1XlHgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x4e, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl1XlHgVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl1XlHgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x4e, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl1XlHgVal(val))
    }
    pub fn write(&mut self, val: Ctrl1XlHgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x4e, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl1XlHgVal(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl1XlHgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x4e, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl1XlHgVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl1XlHgVal) -> Ctrl1XlHgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl1XlHgVal) -> Ctrl1XlHgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl1XlHgVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl1XlHgVal(0x0)).await
    }
}
pub struct Ctrl1XlHgVal(pub u8);
impl Ctrl1XlHgVal {
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
    pub fn xl_hg_regout_en<'a>(&'a mut self) -> FieldXlHgRegoutEn<'a> {
        FieldXlHgRegoutEn(self)
    }
    pub fn hg_usr_off_on_out<'a>(&'a mut self) -> FieldHgUsrOffOnOut<'a> {
        FieldHgUsrOffOnOut(self)
    }
    pub fn odr_xl_hg<'a>(&'a mut self) -> FieldOdrXlHg<'a> {
        FieldOdrXlHg(self)
    }
    pub fn fs_xl_hg<'a>(&'a mut self) -> FieldFsXlHg<'a> {
        FieldFsXlHg(self)
    }
}
pub struct FieldXlHgRegoutEn<'a>(pub &'a mut Ctrl1XlHgVal);
impl<'a> FieldXlHgRegoutEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl1XlHgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl1XlHgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl1XlHgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl1XlHgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldHgUsrOffOnOut<'a>(pub &'a mut Ctrl1XlHgVal);
impl<'a> FieldHgUsrOffOnOut<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl1XlHgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl1XlHgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl1XlHgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl1XlHgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldOdrXlHg<'a>(pub &'a mut Ctrl1XlHgVal);
impl<'a> FieldOdrXlHg<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl1XlHgVal {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl1XlHgVal {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= 0x0 & (!(!0 << 3) << 3);
        self.0
    }
}
pub struct FieldFsXlHg<'a>(pub &'a mut Ctrl1XlHgVal);
impl<'a> FieldFsXlHg<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl1XlHgVal {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl1XlHgVal {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x0 & (!(!0 << 3) << 0);
        self.0
    }
}
