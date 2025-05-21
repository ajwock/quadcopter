use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct ApexConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexConfig1<'a, C> {
    pub fn read(&mut self) -> Result<ApexConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x26, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x26, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig1Val(val))
    }
    pub fn write(&mut self, val: ApexConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x26, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: ApexConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x26, &buf, crate::AccessProc::Standard).await?;
        Ok(())
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
    pub fn smd_enable<'a>(&'a mut self) -> SmdEnable<'a> {
        SmdEnable(self)
    }
    pub fn ff_enable<'a>(&'a mut self) -> FfEnable<'a> {
        FfEnable(self)
    }
    pub fn tilt_enable<'a>(&'a mut self) -> TiltEnable<'a> {
        TiltEnable(self)
    }
    pub fn ped_enable<'a>(&'a mut self) -> PedEnable<'a> {
        PedEnable(self)
    }
    pub fn dmp_odr<'a>(&'a mut self) -> DmpOdr<'a> {
        DmpOdr(self)
    }
}
pub struct SmdEnable<'a>(pub &'a mut ApexConfig1Val);
impl<'a> SmdEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= !(!(val as u8) << 6);
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(false)
    }
}
pub struct FfEnable<'a>(pub &'a mut ApexConfig1Val);
impl<'a> FfEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= !(!(val as u8) << 5);
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(false)
    }
}
pub struct TiltEnable<'a>(pub &'a mut ApexConfig1Val);
impl<'a> TiltEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= !(!(val as u8) << 4);
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(false)
    }
}
pub struct PedEnable<'a>(pub &'a mut ApexConfig1Val);
impl<'a> PedEnable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= !(!(val as u8) << 3);
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig1Val {
        self.assign(false)
    }
}
pub struct DmpOdr<'a>(pub &'a mut ApexConfig1Val);
impl<'a> DmpOdr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig1Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
}
