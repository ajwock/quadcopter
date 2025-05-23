use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct InactivityDur<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> InactivityDur<'a, C> {
    pub fn read(&mut self) -> Result<InactivityDurVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x54, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(InactivityDurVal(val))
    }
    pub async fn read_async(&mut self) -> Result<InactivityDurVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x54, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(InactivityDurVal(val))
    }
    pub fn write(&mut self, val: InactivityDurVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x54, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(InactivityDurVal(raw_val))
    }
    pub async fn write_async(&mut self, val: InactivityDurVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x54, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(InactivityDurVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(InactivityDurVal) -> InactivityDurVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(InactivityDurVal) -> InactivityDurVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(InactivityDurVal(0x4))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(InactivityDurVal(0x4)).await
    }
}
pub struct InactivityDurVal(pub u8);
impl InactivityDurVal {
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
        Self(0x4)
    }
    pub fn sleep_status_on_int<'a>(&'a mut self) -> FieldSleepStatusOnInt<'a> {
        FieldSleepStatusOnInt(self)
    }
    pub fn wu_inact_ths_w<'a>(&'a mut self) -> FieldWuInactThsW<'a> {
        FieldWuInactThsW(self)
    }
    pub fn xl_inact_odr<'a>(&'a mut self) -> FieldXlInactOdr<'a> {
        FieldXlInactOdr(self)
    }
    pub fn inact_dur<'a>(&'a mut self) -> FieldInactDur<'a> {
        FieldInactDur(self)
    }
}
pub struct FieldSleepStatusOnInt<'a>(pub &'a mut InactivityDurVal);
impl<'a> FieldSleepStatusOnInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut InactivityDurVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut InactivityDurVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut InactivityDurVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut InactivityDurVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x4;
        self.0
    }
}
pub struct FieldWuInactThsW<'a>(pub &'a mut InactivityDurVal);
impl<'a> FieldWuInactThsW<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut InactivityDurVal {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut InactivityDurVal {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= 0x4 & (!(!0 << 3) << 4);
        self.0
    }
}
pub struct FieldXlInactOdr<'a>(pub &'a mut InactivityDurVal);
impl<'a> FieldXlInactOdr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut InactivityDurVal {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 2;
        self.0
    }
    pub fn reset(self) -> &'a mut InactivityDurVal {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= 0x4 & (!(!0 << 2) << 2);
        self.0
    }
}
pub struct FieldInactDur<'a>(pub &'a mut InactivityDurVal);
impl<'a> FieldInactDur<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut InactivityDurVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut InactivityDurVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x4 & (!(!0 << 2) << 0);
        self.0
    }
}
