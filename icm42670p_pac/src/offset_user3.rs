use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct OffsetUser3<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> OffsetUser3<'a, D, C> {
    pub fn read(&mut self) -> Result<OffsetUser3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x51, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(OffsetUser3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<OffsetUser3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x51, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(OffsetUser3Val(val))
    }
    pub fn write(&mut self, val: OffsetUser3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x51, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(OffsetUser3Val(raw_val))
    }
    pub async fn write_async(&mut self, val: OffsetUser3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x51, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(OffsetUser3Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(OffsetUser3Val) -> OffsetUser3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(OffsetUser3Val) -> OffsetUser3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(OffsetUser3Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(OffsetUser3Val(0x0)).await
    }
}
pub struct OffsetUser3Val(pub u8);
impl OffsetUser3Val {
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
    pub fn gyro_z_offuser_7_0<'a>(&'a mut self) -> FieldGyroZOffuser70<'a> {
        FieldGyroZOffuser70(self)
    }
}
pub struct FieldGyroZOffuser70<'a>(pub &'a mut OffsetUser3Val);
impl<'a> FieldGyroZOffuser70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
    pub fn set(self, val: u8) -> &'a mut OffsetUser3Val {
        self.0.0 = val;
        self.0
    }
    pub fn reset(self) -> &'a mut OffsetUser3Val {
        self.0.0 = 0x0;
        self.0
    }
}
