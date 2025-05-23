use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct TapSrc<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> TapSrc<'a, C> {
    pub fn read(&mut self) -> Result<TapSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x46, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(TapSrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<TapSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x46, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(TapSrcVal(val))
    }
}
pub struct TapSrcVal(pub u8);
impl TapSrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn tap_ia<'a>(&'a mut self) -> FieldTapIa<'a> {
        FieldTapIa(self)
    }
    pub fn single_tap<'a>(&'a mut self) -> FieldSingleTap<'a> {
        FieldSingleTap(self)
    }
    pub fn double_tap<'a>(&'a mut self) -> FieldDoubleTap<'a> {
        FieldDoubleTap(self)
    }
    pub fn tap_sign<'a>(&'a mut self) -> FieldTapSign<'a> {
        FieldTapSign(self)
    }
    pub fn x_tap<'a>(&'a mut self) -> FieldXTap<'a> {
        FieldXTap(self)
    }
    pub fn y_tap<'a>(&'a mut self) -> FieldYTap<'a> {
        FieldYTap(self)
    }
    pub fn z_tap<'a>(&'a mut self) -> FieldZTap<'a> {
        FieldZTap(self)
    }
}
pub struct FieldTapIa<'a>(pub &'a mut TapSrcVal);
impl<'a> FieldTapIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldSingleTap<'a>(pub &'a mut TapSrcVal);
impl<'a> FieldSingleTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldDoubleTap<'a>(pub &'a mut TapSrcVal);
impl<'a> FieldDoubleTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldTapSign<'a>(pub &'a mut TapSrcVal);
impl<'a> FieldTapSign<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldXTap<'a>(pub &'a mut TapSrcVal);
impl<'a> FieldXTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldYTap<'a>(pub &'a mut TapSrcVal);
impl<'a> FieldYTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldZTap<'a>(pub &'a mut TapSrcVal);
impl<'a> FieldZTap<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
