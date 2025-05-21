use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct DeviceConfig<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> DeviceConfig<'a, C> {
    pub fn read(&mut self) -> Result<DeviceConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x1, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(DeviceConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<DeviceConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x1, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(DeviceConfigVal(val))
    }
    pub fn write(&mut self, val: DeviceConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x1, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: DeviceConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x1, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct DeviceConfigVal(pub u8);
impl DeviceConfigVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn spi_ap_4_wire<'a>(&'a mut self) -> SpiAp4Wire<'a> {
        SpiAp4Wire(self)
    }
    pub fn spi_mode<'a>(&'a mut self) -> SpiMode<'a> {
        SpiMode(self)
    }
}
pub struct SpiAp4Wire<'a>(pub &'a mut DeviceConfigVal);
impl<'a> SpiAp4Wire<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut DeviceConfigVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut DeviceConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut DeviceConfigVal {
        self.assign(false)
    }
}
pub struct SpiMode<'a>(pub &'a mut DeviceConfigVal);
impl<'a> SpiMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut DeviceConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= !(!(val as u8) << 0);
        self.0
    }
    pub fn set_bit(self) -> &'a mut DeviceConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut DeviceConfigVal {
        self.assign(false)
    }
}
