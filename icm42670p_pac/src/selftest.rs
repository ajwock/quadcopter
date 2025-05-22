use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct Selftest<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> Selftest<'a, D, C> {
    pub fn read(&mut self) -> Result<SelftestVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x14, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(SelftestVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SelftestVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x14, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(SelftestVal(val))
    }
    pub fn write(&mut self, val: SelftestVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x14, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(SelftestVal(raw_val))
    }
    pub async fn write_async(&mut self, val: SelftestVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x14, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(SelftestVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(SelftestVal) -> SelftestVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(SelftestVal) -> SelftestVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(SelftestVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(SelftestVal(0x0)).await
    }
}
pub struct SelftestVal(pub u8);
impl SelftestVal {
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
    pub fn gyro_st_en<'a>(&'a mut self) -> FieldGyroStEn<'a> {
        FieldGyroStEn(self)
    }
    pub fn accel_st_en<'a>(&'a mut self) -> FieldAccelStEn<'a> {
        FieldAccelStEn(self)
    }
}
pub struct FieldGyroStEn<'a>(pub &'a mut SelftestVal);
impl<'a> FieldGyroStEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut SelftestVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut SelftestVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut SelftestVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut SelftestVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldAccelStEn<'a>(pub &'a mut SelftestVal);
impl<'a> FieldAccelStEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut SelftestVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut SelftestVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut SelftestVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut SelftestVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
