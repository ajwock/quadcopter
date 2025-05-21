use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct SignalPathReset<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> SignalPathReset<'a, C> {
    pub fn read(&mut self) -> Result<SignalPathResetVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x2, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(SignalPathResetVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SignalPathResetVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x2, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(SignalPathResetVal(val))
    }
    pub fn write(&mut self, val: SignalPathResetVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x2, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: SignalPathResetVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x2, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct SignalPathResetVal(pub u8);
impl SignalPathResetVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn soft_reset_device_config<'a>(&'a mut self) -> SoftResetDeviceConfig<'a> {
        SoftResetDeviceConfig(self)
    }
    pub fn fifo_flush<'a>(&'a mut self) -> FifoFlush<'a> {
        FifoFlush(self)
    }
}
pub struct SoftResetDeviceConfig<'a>(pub &'a mut SignalPathResetVal);
impl<'a> SoftResetDeviceConfig<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut SignalPathResetVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= !(!(val as u8) << 4);
        self.0
    }
    pub fn set_bit(self) -> &'a mut SignalPathResetVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut SignalPathResetVal {
        self.assign(false)
    }
}
pub struct FifoFlush<'a>(pub &'a mut SignalPathResetVal);
impl<'a> FifoFlush<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut SignalPathResetVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut SignalPathResetVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut SignalPathResetVal {
        self.assign(false)
    }
}
