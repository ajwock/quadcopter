use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl7<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl7<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x16, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl7Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x16, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl7Val(val))
    }
    pub fn write(&mut self, val: Ctrl7Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x16, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl7Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl7Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x16, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl7Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl7Val) -> Ctrl7Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl7Val) -> Ctrl7Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl7Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl7Val(0x0)).await
    }
}
pub struct Ctrl7Val(pub u8);
impl Ctrl7Val {
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
    pub fn int1_drdy_xl_hg<'a>(&'a mut self) -> FieldInt1DrdyXlHg<'a> {
        FieldInt1DrdyXlHg(self)
    }
    pub fn int2_drdy_xl_hg<'a>(&'a mut self) -> FieldInt2DrdyXlHg<'a> {
        FieldInt2DrdyXlHg(self)
    }
    pub fn lpf_g_en<'a>(&'a mut self) -> FieldLpfGEn<'a> {
        FieldLpfGEn(self)
    }
}
pub struct FieldInt1DrdyXlHg<'a>(pub &'a mut Ctrl7Val);
impl<'a> FieldInt1DrdyXlHg<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl7Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl7Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldInt2DrdyXlHg<'a>(pub &'a mut Ctrl7Val);
impl<'a> FieldInt2DrdyXlHg<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl7Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl7Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldLpfGEn<'a>(pub &'a mut Ctrl7Val);
impl<'a> FieldLpfGEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl7Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl7Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
