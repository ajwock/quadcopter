use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct WakeUpDur<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> WakeUpDur<'a, C> {
    pub fn read(&mut self) -> Result<WakeUpDurVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x5c, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(WakeUpDurVal(val))
    }
    pub async fn read_async(&mut self) -> Result<WakeUpDurVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x5c, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(WakeUpDurVal(val))
    }
    pub fn write(&mut self, val: WakeUpDurVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x5c, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(WakeUpDurVal(raw_val))
    }
    pub async fn write_async(&mut self, val: WakeUpDurVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x5c, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(WakeUpDurVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(WakeUpDurVal) -> WakeUpDurVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(WakeUpDurVal) -> WakeUpDurVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(WakeUpDurVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(WakeUpDurVal(0x0)).await
    }
}
pub struct WakeUpDurVal(pub u8);
impl WakeUpDurVal {
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
    pub fn ff_dur_5<'a>(&'a mut self) -> FieldFfDur5<'a> {
        FieldFfDur5(self)
    }
    pub fn wake_dur<'a>(&'a mut self) -> FieldWakeDur<'a> {
        FieldWakeDur(self)
    }
    pub fn sleep_dur<'a>(&'a mut self) -> FieldSleepDur<'a> {
        FieldSleepDur(self)
    }
}
pub struct FieldFfDur5<'a>(pub &'a mut WakeUpDurVal);
impl<'a> FieldFfDur5<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut WakeUpDurVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut WakeUpDurVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut WakeUpDurVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut WakeUpDurVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldWakeDur<'a>(pub &'a mut WakeUpDurVal);
impl<'a> FieldWakeDur<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut WakeUpDurVal {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 5;
        self.0
    }
    pub fn reset(self) -> &'a mut WakeUpDurVal {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= 0x0 & (!(!0 << 2) << 5);
        self.0
    }
}
pub struct FieldSleepDur<'a>(pub &'a mut WakeUpDurVal);
impl<'a> FieldSleepDur<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut WakeUpDurVal {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut WakeUpDurVal {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x0 & (!(!0 << 4) << 0);
        self.0
    }
}
