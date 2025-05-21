use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct GyroConfig0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> GyroConfig0<'a, C> {
    pub fn read(&mut self) -> Result<GyroConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x20, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(GyroConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<GyroConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x20, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(GyroConfig0Val(val))
    }
    pub fn write(&mut self, val: GyroConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x20, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: GyroConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x20, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct GyroConfig0Val(pub u8);
impl GyroConfig0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn gyro_ui_fs_sel<'a>(&'a mut self) -> GyroUiFsSel<'a> {
        GyroUiFsSel(self)
    }
    pub fn gyro_odr<'a>(&'a mut self) -> GyroOdr<'a> {
        GyroOdr(self)
    }
}
pub struct GyroUiFsSel<'a>(pub &'a mut GyroConfig0Val);
impl<'a> GyroUiFsSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut GyroConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 5;
        self.0
    }
}
pub struct GyroOdr<'a>(pub &'a mut GyroConfig0Val);
impl<'a> GyroOdr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut GyroConfig0Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
}
