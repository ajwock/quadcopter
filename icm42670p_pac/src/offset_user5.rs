use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct OffsetUser5<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> OffsetUser5<'a, C> {
    pub fn read(&mut self) -> Result<OffsetUser5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x53, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(OffsetUser5Val(val))
    }
    pub async fn read_async(&mut self) -> Result<OffsetUser5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x53, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(OffsetUser5Val(val))
    }
    pub fn write(&mut self, val: OffsetUser5Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x53, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(OffsetUser5Val(raw_val))
    }
    pub async fn write_async(&mut self, val: OffsetUser5Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x53, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(OffsetUser5Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(OffsetUser5Val) -> OffsetUser5Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(OffsetUser5Val) -> OffsetUser5Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(OffsetUser5Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(OffsetUser5Val(0x0)).await
    }
}
pub struct OffsetUser5Val(pub u8);
impl OffsetUser5Val {
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
    pub fn accel_x_offuser_7_0<'a>(&'a mut self) -> FieldAccelXOffuser70<'a> {
        FieldAccelXOffuser70(self)
    }
}
pub struct FieldAccelXOffuser70<'a>(pub &'a mut OffsetUser5Val);
impl<'a> FieldAccelXOffuser70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
    pub fn set(self, val: u8) -> &'a mut OffsetUser5Val {
        self.0.0 = val;
        self.0
    }
    pub fn reset(self) -> &'a mut OffsetUser5Val {
        self.0.0 = 0x0;
        self.0
    }
}
