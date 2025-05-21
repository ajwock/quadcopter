use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct WomConfig<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> WomConfig<'a, C> {
    pub fn read(&mut self) -> Result<WomConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x27, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(WomConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<WomConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x27, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(WomConfigVal(val))
    }
    pub fn write(&mut self, val: WomConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x27, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: WomConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x27, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct WomConfigVal(pub u8);
impl WomConfigVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn wom_int_dur<'a>(&'a mut self) -> WomIntDur<'a> {
        WomIntDur(self)
    }
    pub fn wom_int_mode<'a>(&'a mut self) -> WomIntMode<'a> {
        WomIntMode(self)
    }
    pub fn wom_mode<'a>(&'a mut self) -> WomMode<'a> {
        WomMode(self)
    }
    pub fn wom_en<'a>(&'a mut self) -> WomEn<'a> {
        WomEn(self)
    }
}
pub struct WomIntDur<'a>(pub &'a mut WomConfigVal);
impl<'a> WomIntDur<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut WomConfigVal {
        self.0.0 &= !(!(!0 << 2) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 3;
        self.0
    }
}
pub struct WomIntMode<'a>(pub &'a mut WomConfigVal);
impl<'a> WomIntMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut WomConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WomConfigVal {
        self.assign(false)
    }
}
pub struct WomMode<'a>(pub &'a mut WomConfigVal);
impl<'a> WomMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= !(!(val as u8) << 1);
        self.0
    }
    pub fn set_bit(self) -> &'a mut WomConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WomConfigVal {
        self.assign(false)
    }
}
pub struct WomEn<'a>(pub &'a mut WomConfigVal);
impl<'a> WomEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WomConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= !(!(val as u8) << 0);
        self.0
    }
    pub fn set_bit(self) -> &'a mut WomConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WomConfigVal {
        self.assign(false)
    }
}
