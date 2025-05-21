use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct ApexConfig0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexConfig0<'a, C> {
    pub fn read(&mut self) -> Result<ApexConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x25, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x25, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig0Val(val))
    }
    pub fn write(&mut self, val: ApexConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x25, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: ApexConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x25, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct ApexConfig0Val(pub u8);
impl ApexConfig0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn dmp_power_save_en<'a>(&'a mut self) -> DmpPowerSaveEn<'a> {
        DmpPowerSaveEn(self)
    }
    pub fn dmp_init_en<'a>(&'a mut self) -> DmpInitEn<'a> {
        DmpInitEn(self)
    }
    pub fn dmp_mem_reset_en<'a>(&'a mut self) -> DmpMemResetEn<'a> {
        DmpMemResetEn(self)
    }
}
pub struct DmpPowerSaveEn<'a>(pub &'a mut ApexConfig0Val);
impl<'a> DmpPowerSaveEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= !(!(val as u8) << 3);
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(false)
    }
}
pub struct DmpInitEn<'a>(pub &'a mut ApexConfig0Val);
impl<'a> DmpInitEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(false)
    }
}
pub struct DmpMemResetEn<'a>(pub &'a mut ApexConfig0Val);
impl<'a> DmpMemResetEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut ApexConfig0Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= !(!(val as u8) << 0);
        self.0
    }
    pub fn set_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut ApexConfig0Val {
        self.assign(false)
    }
}
