use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct D6DSrc<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> D6DSrc<'a, C> {
    pub fn read(&mut self) -> Result<D6DSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x47, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(D6DSrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<D6DSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x47, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(D6DSrcVal(val))
    }
}
pub struct D6DSrcVal(pub u8);
impl D6DSrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn d6_d_ia<'a>(&'a mut self) -> FieldD6DIa<'a> {
        FieldD6DIa(self)
    }
    pub fn zh<'a>(&'a mut self) -> FieldZh<'a> {
        FieldZh(self)
    }
    pub fn zl<'a>(&'a mut self) -> FieldZl<'a> {
        FieldZl(self)
    }
    pub fn yh<'a>(&'a mut self) -> FieldYh<'a> {
        FieldYh(self)
    }
    pub fn yl<'a>(&'a mut self) -> FieldYl<'a> {
        FieldYl(self)
    }
    pub fn xh<'a>(&'a mut self) -> FieldXh<'a> {
        FieldXh(self)
    }
    pub fn xl<'a>(&'a mut self) -> FieldXl<'a> {
        FieldXl(self)
    }
}
pub struct FieldD6DIa<'a>(pub &'a mut D6DSrcVal);
impl<'a> FieldD6DIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldZh<'a>(pub &'a mut D6DSrcVal);
impl<'a> FieldZh<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldZl<'a>(pub &'a mut D6DSrcVal);
impl<'a> FieldZl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldYh<'a>(pub &'a mut D6DSrcVal);
impl<'a> FieldYh<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldYl<'a>(pub &'a mut D6DSrcVal);
impl<'a> FieldYl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldXh<'a>(pub &'a mut D6DSrcVal);
impl<'a> FieldXh<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldXl<'a>(pub &'a mut D6DSrcVal);
impl<'a> FieldXl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
