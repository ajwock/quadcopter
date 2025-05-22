use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct SensorConfig3<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> SensorConfig3<'a, D, C> {
    pub fn read(&mut self) -> Result<SensorConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x6, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(SensorConfig3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<SensorConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x6, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(SensorConfig3Val(val))
    }
    pub fn write(&mut self, val: SensorConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x6, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(SensorConfig3Val(raw_val))
    }
    pub async fn write_async(&mut self, val: SensorConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x6, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(SensorConfig3Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(SensorConfig3Val) -> SensorConfig3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(SensorConfig3Val) -> SensorConfig3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(SensorConfig3Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(SensorConfig3Val(0x0)).await
    }
}
pub struct SensorConfig3Val(pub u8);
impl SensorConfig3Val {
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
    pub fn apex_disable<'a>(&'a mut self) -> FieldApexDisable<'a> {
        FieldApexDisable(self)
    }
}
pub struct FieldApexDisable<'a>(pub &'a mut SensorConfig3Val);
impl<'a> FieldApexDisable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut SensorConfig3Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut SensorConfig3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut SensorConfig3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut SensorConfig3Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
