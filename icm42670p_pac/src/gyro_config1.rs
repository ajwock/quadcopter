use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct GyroConfig1<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> GyroConfig1<'a, D, C> {
    pub fn read(&mut self) -> Result<GyroConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x23, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(GyroConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<GyroConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x23, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(GyroConfig1Val(val))
    }
    pub fn write(&mut self, val: GyroConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x23, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(GyroConfig1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: GyroConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x23, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(GyroConfig1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(GyroConfig1Val) -> GyroConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(GyroConfig1Val) -> GyroConfig1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(GyroConfig1Val(0x31))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(GyroConfig1Val(0x31)).await
    }
}
pub struct GyroConfig1Val(pub u8);
impl GyroConfig1Val {
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
        Self(0x31)
    }
    pub fn gyro_ui_filt_bw<'a>(&'a mut self) -> FieldGyroUiFiltBw<'a> {
        FieldGyroUiFiltBw(self)
    }
}
pub struct FieldGyroUiFiltBw<'a>(pub &'a mut GyroConfig1Val);
impl<'a> FieldGyroUiFiltBw<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut GyroConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut GyroConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= 0x31 & (!(!0 << 3) << 0);
        self.0
    }
}
