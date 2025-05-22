use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntSource4<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource4<'a, C> {
    pub fn read(&mut self) -> Result<IntSource4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x2e, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource4Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x2e, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource4Val(val))
    }
    pub fn write(&mut self, val: IntSource4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x2e, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntSource4Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntSource4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x2e, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntSource4Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntSource4Val) -> IntSource4Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntSource4Val) -> IntSource4Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntSource4Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntSource4Val(0x0)).await
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
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn i32_protocol_error_int2_en<'a>(&'a mut self) -> FieldI32ProtocolErrorInt2En<'a> {
        FieldI32ProtocolErrorInt2En(self)
    }
    pub fn smd_int2_en<'a>(&'a mut self) -> FieldSmdInt2En<'a> {
        FieldSmdInt2En(self)
    }
    pub fn wom_z_int2_en<'a>(&'a mut self) -> FieldWomZInt2En<'a> {
        FieldWomZInt2En(self)
    }
    pub fn wom_y_int2_en<'a>(&'a mut self) -> FieldWomYInt2En<'a> {
        FieldWomYInt2En(self)
    }
    pub fn wom_x_int2_en<'a>(&'a mut self) -> FieldWomXInt2En<'a> {
        FieldWomXInt2En(self)
    }
}
pub struct FieldI32ProtocolErrorInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> FieldI32ProtocolErrorInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldSmdInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> FieldSmdInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldWomZInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> FieldWomZInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldWomYInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> FieldWomYInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldWomXInt2En<'a>(pub &'a mut IntSource4Val);
impl<'a> FieldWomXInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource4Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource4Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource4Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
