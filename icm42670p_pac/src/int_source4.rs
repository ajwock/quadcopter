use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntSource4<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource4<'a, C> {
    pub fn read(&mut self) -> Result<IntSource4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x2e, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource4Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x2e, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource4Val(val))
    }
    pub fn write(&mut self, val: IntSource4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x2e, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: IntSource4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x2e, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct IntSource4Val(pub u8);
impl IntSource4Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn i32_protocol_error_int2_en<'a>(&'a mut self) -> I32ProtocolErrorInt2En<'a> {
        I32ProtocolErrorInt2En(self)
    }
    pub fn smd_int2_en<'a>(&'a mut self) -> SmdInt2En<'a> {
        SmdInt2En(self)
    }
    pub fn wom_z_int2_en<'a>(&'a mut self) -> WomZInt2En<'a> {
        WomZInt2En(self)
    }
    pub fn wom_y_int2_en<'a>(&'a mut self) -> WomYInt2En<'a> {
        WomYInt2En(self)
    }
    pub fn wom_x_int2_en<'a>(&'a mut self) -> WomXInt2En<'a> {
        WomXInt2En(self)
    }
}
pub struct I32ProtocolErrorInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> I32ProtocolErrorInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= !(!(val as u8) << 6);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
}
pub struct SmdInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> SmdInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= !(!(val as u8) << 3);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
}
pub struct WomZInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> WomZInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
}
pub struct WomYInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> WomYInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= !(!(val as u8) << 1);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
}
pub struct WomXInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> WomXInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= !(!(val as u8) << 0);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
}
