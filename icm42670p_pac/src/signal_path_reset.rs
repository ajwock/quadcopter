use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct SignalPathReset<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> SignalPathReset<'a, D, C> {
    pub fn read(&mut self) -> Result<SignalPathResetVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x2, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(SignalPathResetVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SignalPathResetVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x2, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(SignalPathResetVal(val))
    }
    pub fn write(&mut self, val: SignalPathResetVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x2, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(SignalPathResetVal(raw_val))
    }
    pub async fn write_async(&mut self, val: SignalPathResetVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x2, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(SignalPathResetVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(SignalPathResetVal) -> SignalPathResetVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(SignalPathResetVal) -> SignalPathResetVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(SignalPathResetVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(SignalPathResetVal(0x0)).await
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
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn soft_reset_device_config<'a>(&'a mut self) -> FieldSoftResetDeviceConfig<'a> {
        FieldSoftResetDeviceConfig(self)
    }
    pub fn fifo_flush<'a>(&'a mut self) -> FieldFifoFlush<'a> {
        FieldFifoFlush(self)
    }
}
pub struct FieldSoftResetDeviceConfig<'a>(pub &'a mut SignalPathResetVal);
impl<'a> FieldSoftResetDeviceConfig<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut SignalPathResetVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut SignalPathResetVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut SignalPathResetVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut SignalPathResetVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldFifoFlush<'a>(pub &'a mut SignalPathResetVal);
impl<'a> FieldFifoFlush<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut SignalPathResetVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut SignalPathResetVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut SignalPathResetVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut SignalPathResetVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
