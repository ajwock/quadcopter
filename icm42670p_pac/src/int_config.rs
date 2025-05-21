use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntConfig<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntConfig<'a, C> {
    pub fn read(&mut self) -> Result<IntConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x6, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<IntConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x6, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntConfigVal(val))
    }
    pub fn write(&mut self, val: IntConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x6, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: IntConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x6, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct IntConfigVal(pub u8);
impl IntConfigVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn int2_mode<'a>(&'a mut self) -> Int2Mode<'a> {
        Int2Mode(self)
    }
    pub fn int2_drive_circuit<'a>(&'a mut self) -> Int2DriveCircuit<'a> {
        Int2DriveCircuit(self)
    }
    pub fn int2_polarity<'a>(&'a mut self) -> Int2Polarity<'a> {
        Int2Polarity(self)
    }
    pub fn int1_mode<'a>(&'a mut self) -> Int1Mode<'a> {
        Int1Mode(self)
    }
    pub fn int1_drive_circuit<'a>(&'a mut self) -> Int1DriveCircuit<'a> {
        Int1DriveCircuit(self)
    }
    pub fn int1_polarity<'a>(&'a mut self) -> Int1Polarity<'a> {
        Int1Polarity(self)
    }
}
pub struct Int2Mode<'a>(pub &'a mut IntConfigVal);
impl<'a> Int2Mode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= !(!(val as u8) << 5);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
}
pub struct Int2DriveCircuit<'a>(pub &'a mut IntConfigVal);
impl<'a> Int2DriveCircuit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= !(!(val as u8) << 4);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
}
pub struct Int2Polarity<'a>(pub &'a mut IntConfigVal);
impl<'a> Int2Polarity<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= !(!(val as u8) << 3);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
}
pub struct Int1Mode<'a>(pub &'a mut IntConfigVal);
impl<'a> Int1Mode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
}
pub struct Int1DriveCircuit<'a>(pub &'a mut IntConfigVal);
impl<'a> Int1DriveCircuit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= !(!(val as u8) << 1);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
}
pub struct Int1Polarity<'a>(pub &'a mut IntConfigVal);
impl<'a> Int1Polarity<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= !(!(val as u8) << 0);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
}
