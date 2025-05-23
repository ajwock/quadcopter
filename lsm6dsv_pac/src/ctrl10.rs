use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl10<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl10<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl10Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x19, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl10Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl10Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x19, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl10Val(val))
    }
    pub fn write(&mut self, val: Ctrl10Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x19, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl10Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl10Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x19, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl10Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl10Val) -> Ctrl10Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl10Val) -> Ctrl10Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl10Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl10Val(0x0)).await
    }
}
pub struct Ctrl10Val(pub u8);
impl Ctrl10Val {
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
    pub fn emb_func_debug<'a>(&'a mut self) -> FieldEmbFuncDebug<'a> {
        FieldEmbFuncDebug(self)
    }
    pub fn st_g<'a>(&'a mut self) -> FieldStG<'a> {
        FieldStG(self)
    }
    pub fn st_xl<'a>(&'a mut self) -> FieldStXl<'a> {
        FieldStXl(self)
    }
}
pub struct FieldEmbFuncDebug<'a>(pub &'a mut Ctrl10Val);
impl<'a> FieldEmbFuncDebug<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl10Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl10Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl10Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl10Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldStG<'a>(pub &'a mut Ctrl10Val);
impl<'a> FieldStG<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl10Val {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 2;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl10Val {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= 0x0 & (!(!0 << 2) << 2);
        self.0
    }
}
pub struct FieldStXl<'a>(pub &'a mut Ctrl10Val);
impl<'a> FieldStXl<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl10Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl10Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x0 & (!(!0 << 2) << 0);
        self.0
    }
}
