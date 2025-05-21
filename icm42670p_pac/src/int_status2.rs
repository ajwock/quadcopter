use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntStatus2<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntStatus2<'a, C> {
    pub fn read(&mut self) -> Result<IntStatus2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x3b, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatus2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntStatus2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x3b, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatus2Val(val))
    }
}
pub struct IntStatus2Val(pub u8);
impl IntStatus2Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn smd_int<'a>(&'a mut self) -> SmdInt<'a> {
        SmdInt(self)
    }
    pub fn wom_x_int<'a>(&'a mut self) -> WomXInt<'a> {
        WomXInt(self)
    }
    pub fn wom_y_int<'a>(&'a mut self) -> WomYInt<'a> {
        WomYInt(self)
    }
    pub fn wom_z_int<'a>(&'a mut self) -> WomZInt<'a> {
        WomZInt(self)
    }
}
pub struct SmdInt<'a>(pub &'a mut IntStatus2Val);
impl<'a> SmdInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct WomXInt<'a>(pub &'a mut IntStatus2Val);
impl<'a> WomXInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct WomYInt<'a>(pub &'a mut IntStatus2Val);
impl<'a> WomYInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct WomZInt<'a>(pub &'a mut IntStatus2Val);
impl<'a> WomZInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
