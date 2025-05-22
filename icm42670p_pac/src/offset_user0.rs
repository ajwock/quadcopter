use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct OffsetUser0<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> OffsetUser0<'a, D, C> {
    pub fn read(&mut self) -> Result<OffsetUser0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x4e, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(OffsetUser0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<OffsetUser0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x4e, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(OffsetUser0Val(val))
    }
    pub fn write(&mut self, val: OffsetUser0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x4e, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(OffsetUser0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: OffsetUser0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x4e, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(OffsetUser0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(OffsetUser0Val) -> OffsetUser0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(OffsetUser0Val) -> OffsetUser0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(OffsetUser0Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(OffsetUser0Val(0x0)).await
    }
}
pub struct OffsetUser0Val(pub u8);
impl OffsetUser0Val {
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
    pub fn gyro_x_offuser_7_0<'a>(&'a mut self) -> FieldGyroXOffuser70<'a> {
        FieldGyroXOffuser70(self)
    }
}
pub struct FieldGyroXOffuser70<'a>(pub &'a mut OffsetUser0Val);
impl<'a> FieldGyroXOffuser70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
    pub fn set(self, val: u8) -> &'a mut OffsetUser0Val {
        self.0.0 = val;
        self.0
    }
    pub fn reset(self) -> &'a mut OffsetUser0Val {
        self.0.0 = 0x0;
        self.0
    }
}
