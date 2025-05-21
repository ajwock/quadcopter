use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct GyroConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> GyroConfig1<'a, C> {
    pub fn read(&mut self) -> Result<GyroConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x23, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(GyroConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<GyroConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x23, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(GyroConfig1Val(val))
    }
    pub fn write(&mut self, val: GyroConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x23, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: GyroConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x23, &buf, crate::AccessProc::Standard).await?;
        Ok(())
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
    pub fn gyro_ui_filt_bw<'a>(&'a mut self) -> GyroUiFiltBw<'a> {
        GyroUiFiltBw(self)
    }
}
pub struct GyroUiFiltBw<'a>(pub &'a mut GyroConfig1Val);
impl<'a> GyroUiFiltBw<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut GyroConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
}
