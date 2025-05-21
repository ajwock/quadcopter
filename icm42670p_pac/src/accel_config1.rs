use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct AccelConfig1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> AccelConfig1<'a, C> {
    pub fn read(&mut self) -> Result<AccelConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x24, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelConfig1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<AccelConfig1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x24, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(AccelConfig1Val(val))
    }
    pub fn write(&mut self, val: AccelConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x24, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: AccelConfig1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x24, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct AccelConfig1Val(pub u8);
impl AccelConfig1Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn accel_ui_avg<'a>(&'a mut self) -> AccelUiAvg<'a> {
        AccelUiAvg(self)
    }
    pub fn accel_ui_filt_bw<'a>(&'a mut self) -> AccelUiFiltBw<'a> {
        AccelUiFiltBw(self)
    }
}
pub struct AccelUiAvg<'a>(pub &'a mut AccelConfig1Val);
impl<'a> AccelUiAvg<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut AccelConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
}
pub struct AccelUiFiltBw<'a>(pub &'a mut AccelConfig1Val);
impl<'a> AccelUiFiltBw<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut AccelConfig1Val {
        self.0.0 &= !(!(!0 << 3) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 0;
        self.0
    }
}
