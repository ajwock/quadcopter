use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntSource9<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource9<'a, C> {
    pub fn read(&mut self) -> Result<IntSource9Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x32, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource9Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource9Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x32, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource9Val(val))
    }
    pub fn write(&mut self, val: IntSource9Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x32, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntSource9Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntSource9Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x32, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntSource9Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntSource9Val) -> IntSource9Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntSource9Val) -> IntSource9Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntSource9Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntSource9Val(0x0)).await
    }
}
pub struct IntSource9Val(pub u8);
impl IntSource9Val {
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
    pub fn i3_c_protocol_error_ibi_en<'a>(&'a mut self) -> FieldI3CProtocolErrorIbiEn<'a> {
        FieldI3CProtocolErrorIbiEn(self)
    }
    pub fn ff_ibi_en<'a>(&'a mut self) -> FieldFfIbiEn<'a> {
        FieldFfIbiEn(self)
    }
    pub fn lowg_ibi_en<'a>(&'a mut self) -> FieldLowgIbiEn<'a> {
        FieldLowgIbiEn(self)
    }
    pub fn smd_ibi_en<'a>(&'a mut self) -> FieldSmdIbiEn<'a> {
        FieldSmdIbiEn(self)
    }
    pub fn wom_z_ibi_en<'a>(&'a mut self) -> FieldWomZIbiEn<'a> {
        FieldWomZIbiEn(self)
    }
    pub fn wom_y_ibi_en<'a>(&'a mut self) -> FieldWomYIbiEn<'a> {
        FieldWomYIbiEn(self)
    }
    pub fn wom_x_ibi_en<'a>(&'a mut self) -> FieldWomXIbiEn<'a> {
        FieldWomXIbiEn(self)
    }
    pub fn st_done_ibi_en<'a>(&'a mut self) -> FieldStDoneIbiEn<'a> {
        FieldStDoneIbiEn(self)
    }
}
pub struct FieldI3CProtocolErrorIbiEn<'a>(pub &'a mut IntSource9Val);
impl<'a> FieldI3CProtocolErrorIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldFfIbiEn<'a>(pub &'a mut IntSource9Val);
impl<'a> FieldFfIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldLowgIbiEn<'a>(pub &'a mut IntSource9Val);
impl<'a> FieldLowgIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldSmdIbiEn<'a>(pub &'a mut IntSource9Val);
impl<'a> FieldSmdIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldWomZIbiEn<'a>(pub &'a mut IntSource9Val);
impl<'a> FieldWomZIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldWomYIbiEn<'a>(pub &'a mut IntSource9Val);
impl<'a> FieldWomYIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldWomXIbiEn<'a>(pub &'a mut IntSource9Val);
impl<'a> FieldWomXIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldStDoneIbiEn<'a>(pub &'a mut IntSource9Val);
impl<'a> FieldStDoneIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource9Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource9Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntSource9Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
