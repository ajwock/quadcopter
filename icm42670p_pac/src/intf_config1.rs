use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntfConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntfConfig1<'a, C> {
    pub fn read(&mut self) -> Result<IntfConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x36, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntfConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x36, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig1Val(val))
    }
    pub fn write(&mut self, val: IntfConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x36, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: IntfConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x36, &buf, crate::AccessProc::Standard).await?;
        Ok(())
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
    pub fn i3_c_sdr_en<'a>(&'a mut self) -> I3CSdrEn<'a> {
        I3CSdrEn(self)
    }
    pub fn i3_c_ddr_en<'a>(&'a mut self) -> I3CDdrEn<'a> {
        I3CDdrEn(self)
    }
    pub fn clksel<'a>(&'a mut self) -> Clksel<'a> {
        Clksel(self)
    }
}
pub struct I3CSdrEn<'a>(pub &'a mut IntfConfig1Val);
impl<'a> I3CSdrEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= !(!(val as u8) << 3);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig1Val {
        self.assign(false)
    }
}
pub struct I3CDdrEn<'a>(pub &'a mut IntfConfig1Val);
impl<'a> I3CDdrEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig1Val {
        self.assign(false)
    }
}
pub struct Clksel<'a>(pub &'a mut IntfConfig1Val);
impl<'a> Clksel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut IntfConfig1Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
}
