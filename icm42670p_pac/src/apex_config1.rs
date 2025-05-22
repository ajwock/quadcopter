use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig1<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexConfig1<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x26, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x26, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig1Val(val))
    }
    pub fn write(&mut self, val: ApexConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x26, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x26, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig1Val) -> ApexConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig1Val) -> ApexConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig1Val(0x2))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig1Val(0x2)).await
    }
}
pub struct ApexConfig1Val(pub u8);
impl ApexConfig1Val {
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
        Self(0x2)
    }
    pub fn smd_enable<'a>(&'a mut self) -> FieldSmdEnable<'a> {
        FieldSmdEnable(self)
    }
    pub fn ff_enable<'a>(&'a mut self) -> FieldFfEnable<'a> {
        FieldFfEnable(self)
    }
    pub fn tilt_enable<'a>(&'a mut self) -> FieldTiltEnable<'a> {
        FieldTiltEnable(self)
    }
    pub fn ped_enable<'a>(&'a mut self) -> FieldPedEnable<'a> {
        FieldPedEnable(self)
    }
    pub fn dmp_odr<'a>(&'a mut self) -> FieldDmpOdr<'a> {
        FieldDmpOdr(self)
    }
}
pub struct FieldSmdEnable<'a>(pub &'a mut ApexConfig1Val);
impl<'a> FieldSmdEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x2;
        self.0
    }
}
pub struct FieldFfEnable<'a>(pub &'a mut ApexConfig1Val);
impl<'a> FieldFfEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x2;
        self.0
    }
}
pub struct FieldTiltEnable<'a>(pub &'a mut ApexConfig1Val);
impl<'a> FieldTiltEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x2;
        self.0
    }
}
pub struct FieldPedEnable<'a>(pub &'a mut ApexConfig1Val);
impl<'a> FieldPedEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x2;
        self.0
    }
}
pub struct FieldDmpOdr<'a>(pub &'a mut ApexConfig1Val);
impl<'a> FieldDmpOdr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x2 & (!(!0 << 2) << 0);
        self.0
    }
}
