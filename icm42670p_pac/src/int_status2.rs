use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntStatus2<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> IntStatus2<'a, D, C> {
    pub fn read(&mut self) -> Result<IntStatus2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x3b, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatus2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntStatus2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x3b, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatus2Val(val))
    }
}
pub struct IntStatus2Val(pub u8);
impl IntStatus2Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn smd_int<'a>(&'a mut self) -> FieldSmdInt<'a> {
        FieldSmdInt(self)
    }
    pub fn wom_x_int<'a>(&'a mut self) -> FieldWomXInt<'a> {
        FieldWomXInt(self)
    }
    pub fn wom_y_int<'a>(&'a mut self) -> FieldWomYInt<'a> {
        FieldWomYInt(self)
    }
    pub fn wom_z_int<'a>(&'a mut self) -> FieldWomZInt<'a> {
        FieldWomZInt(self)
    }
}
pub struct FieldSmdInt<'a>(pub &'a mut IntStatus2Val);
impl<'a> FieldSmdInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldWomXInt<'a>(pub &'a mut IntStatus2Val);
impl<'a> FieldWomXInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldWomYInt<'a>(pub &'a mut IntStatus2Val);
impl<'a> FieldWomYInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldWomZInt<'a>(pub &'a mut IntStatus2Val);
impl<'a> FieldWomZInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
