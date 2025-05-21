use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct FifoConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> FifoConfig1<'a, C> {
    pub fn read(&mut self) -> Result<FifoConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x28, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x28, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig1Val(val))
    }
    pub fn write(&mut self, val: FifoConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x28, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: FifoConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x28, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct FifoConfig1Val(pub u8);
impl FifoConfig1Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn fifo_mode<'a>(&'a mut self) -> FifoMode<'a> {
        FifoMode(self)
    }
    pub fn fifo_bypass<'a>(&'a mut self) -> FifoBypass<'a> {
        FifoBypass(self)
    }
}
pub struct FifoMode<'a>(pub &'a mut FifoConfig1Val);
impl<'a> FifoMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig1Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= !(!(val as u8) << 1);
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig1Val {
        self.assign(false)
    }
}
pub struct FifoBypass<'a>(pub &'a mut FifoConfig1Val);
impl<'a> FifoBypass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig1Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= !(!(val as u8) << 0);
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig1Val {
        self.assign(false)
    }
}
