use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntfConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntfConfig1<'a, C> {
    pub fn read(&mut self) -> Result<IntfConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x36, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntfConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x36, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig1Val(val))
    }
    pub fn write(&mut self, val: IntfConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x36, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntfConfig1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntfConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x36, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntfConfig1Val) -> IntfConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntfConfig1Val) -> IntfConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntfConfig1Val(0x4d))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig1Val(0x4d)).await
    }
}
pub struct IntfConfig1Val(pub u8);
impl IntfConfig1Val {
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
        Self(0x4d)
    }
    pub fn i3_c_sdr_en<'a>(&'a mut self) -> FieldI3CSdrEn<'a> {
        FieldI3CSdrEn(self)
    }
    pub fn i3_c_ddr_en<'a>(&'a mut self) -> FieldI3CDdrEn<'a> {
        FieldI3CDdrEn(self)
    }
    pub fn clksel<'a>(&'a mut self) -> FieldClksel<'a> {
        FieldClksel(self)
    }
}
pub struct FieldI3CSdrEn<'a>(pub &'a mut IntfConfig1Val);
impl<'a> FieldI3CSdrEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x4d;
        self.0
    }
}
pub struct FieldI3CDdrEn<'a>(pub &'a mut IntfConfig1Val);
impl<'a> FieldI3CDdrEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x4d;
        self.0
    }
}
pub struct FieldClksel<'a>(pub &'a mut IntfConfig1Val);
impl<'a> FieldClksel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x4d & (!(!0 << 2) << 0);
        self.0
    }
}
