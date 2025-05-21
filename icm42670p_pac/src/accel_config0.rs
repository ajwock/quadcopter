use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct AccelConfig0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> AccelConfig0<'a, C> {
    pub fn read(&mut self) -> Result<AccelConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x21, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<AccelConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x21, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelConfig0Val(val))
    }
    pub fn write(&mut self, val: AccelConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x21, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: AccelConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x21, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct AccelConfig0Val(pub u8);
impl AccelConfig0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn accel_ui_fs_sel<'a>(&'a mut self) -> AccelUiFsSel<'a> {
        AccelUiFsSel(self)
    }
    pub fn accel_odr<'a>(&'a mut self) -> AccelOdr<'a> {
        AccelOdr(self)
    }
}
pub struct AccelUiFsSel<'a>(pub &'a mut AccelConfig0Val);
impl<'a> AccelUiFsSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut AccelConfig0Val {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 5;
        self.0
    }
}
pub struct AccelOdr<'a>(pub &'a mut AccelConfig0Val);
impl<'a> AccelOdr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut AccelConfig0Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
}
