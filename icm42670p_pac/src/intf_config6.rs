use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntfConfig6<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> IntfConfig6<'a, D, C> {
    pub fn read(&mut self) -> Result<IntfConfig6Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x23, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig6Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntfConfig6Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x23, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig6Val(val))
    }
    pub fn write(&mut self, val: IntfConfig6Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x23, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntfConfig6Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntfConfig6Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x23, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig6Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntfConfig6Val) -> IntfConfig6Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntfConfig6Val) -> IntfConfig6Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntfConfig6Val(0x7c))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig6Val(0x7c)).await
    }
}
pub struct IntfConfig6Val(pub u8);
impl IntfConfig6Val {
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
        Self(0x7c)
    }
    pub fn i3_c_timeout_en<'a>(&'a mut self) -> FieldI3CTimeoutEn<'a> {
        FieldI3CTimeoutEn(self)
    }
    pub fn i3_c_ibi_byte_en<'a>(&'a mut self) -> FieldI3CIbiByteEn<'a> {
        FieldI3CIbiByteEn(self)
    }
    pub fn i3_c_ibi_en<'a>(&'a mut self) -> FieldI3CIbiEn<'a> {
        FieldI3CIbiEn(self)
    }
}
pub struct FieldI3CTimeoutEn<'a>(pub &'a mut IntfConfig6Val);
impl<'a> FieldI3CTimeoutEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig6Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig6Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig6Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig6Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x7c;
        self.0
    }
}
pub struct FieldI3CIbiByteEn<'a>(pub &'a mut IntfConfig6Val);
impl<'a> FieldI3CIbiByteEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig6Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig6Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig6Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig6Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x7c;
        self.0
    }
}
pub struct FieldI3CIbiEn<'a>(pub &'a mut IntfConfig6Val);
impl<'a> FieldI3CIbiEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig6Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig6Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig6Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig6Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x7c;
        self.0
    }
}
