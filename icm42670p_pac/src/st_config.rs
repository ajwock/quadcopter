use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct StConfig<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> StConfig<'a, C> {
    pub fn read(&mut self) -> Result<StConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x13, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(StConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<StConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x13, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(StConfigVal(val))
    }
    pub fn write(&mut self, val: StConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x13, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(StConfigVal(raw_val))
    }
    pub async fn write_async(&mut self, val: StConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x13, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(StConfigVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(StConfigVal) -> StConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(StConfigVal) -> StConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(StConfigVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(StConfigVal(0x0)).await
    }
}
pub struct StConfigVal(pub u8);
impl StConfigVal {
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
    pub fn st_number_sample<'a>(&'a mut self) -> FieldStNumberSample<'a> {
        FieldStNumberSample(self)
    }
    pub fn accel_st_lim<'a>(&'a mut self) -> FieldAccelStLim<'a> {
        FieldAccelStLim(self)
    }
    pub fn gyro_st_lim<'a>(&'a mut self) -> FieldGyroStLim<'a> {
        FieldGyroStLim(self)
    }
}
pub struct FieldStNumberSample<'a>(pub &'a mut StConfigVal);
impl<'a> FieldStNumberSample<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut StConfigVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut StConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut StConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut StConfigVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldAccelStLim<'a>(pub &'a mut StConfigVal);
impl<'a> FieldAccelStLim<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut StConfigVal {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
    pub fn reset(self) -> &'a mut StConfigVal {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= 0x0 & (!(!0 << 3) << 3);
        self.0
    }
}
pub struct FieldGyroStLim<'a>(pub &'a mut StConfigVal);
impl<'a> FieldGyroStLim<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut StConfigVal {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut StConfigVal {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x0 & (!(!0 << 3) << 0);
        self.0
    }
}
