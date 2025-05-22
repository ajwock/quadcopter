use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntSource1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource1<'a, C> {
    pub fn read(&mut self) -> Result<IntSource1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x2c, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x2c, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource1Val(val))
    }
    pub fn write(&mut self, val: IntSource1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x2c, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntSource1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntSource1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x2c, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntSource1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntSource1Val) -> IntSource1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntSource1Val) -> IntSource1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntSource1Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntSource1Val(0x0)).await
    }
}
pub struct IntSource1Val(pub u8);
impl IntSource1Val {
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
    pub fn i32_protocol_error_int1_en<'a>(&'a mut self) -> FieldI32ProtocolErrorInt1En<'a> {
        FieldI32ProtocolErrorInt1En(self)
    }
    pub fn smd_int1_en<'a>(&'a mut self) -> FieldSmdInt1En<'a> {
        FieldSmdInt1En(self)
    }
    pub fn wom_z_int1_en<'a>(&'a mut self) -> FieldWomZInt1En<'a> {
        FieldWomZInt1En(self)
    }
    pub fn wom_y_int1_en<'a>(&'a mut self) -> FieldWomYInt1En<'a> {
        FieldWomYInt1En(self)
    }
    pub fn wom_x_int1_en<'a>(&'a mut self) -> FieldWomXInt1En<'a> {
        FieldWomXInt1En(self)
    }
}
pub struct FieldI32ProtocolErrorInt1En<'a>(pub &'a mut IntSource1Val);
impl<'a> FieldI32ProtocolErrorInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldSmdInt1En<'a>(pub &'a mut IntSource1Val);
impl<'a> FieldSmdInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldWomZInt1En<'a>(pub &'a mut IntSource1Val);
impl<'a> FieldWomZInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldWomYInt1En<'a>(pub &'a mut IntSource1Val);
impl<'a> FieldWomYInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldWomXInt1En<'a>(pub &'a mut IntSource1Val);
impl<'a> FieldWomXInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource1Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
