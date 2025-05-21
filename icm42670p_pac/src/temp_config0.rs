use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct TempConfig0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> TempConfig0<'a, C> {
    pub fn read(&mut self) -> Result<TempConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x22, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(TempConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<TempConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x22, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(TempConfig0Val(val))
    }
    pub fn write(&mut self, val: TempConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x22, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: TempConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x22, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct TempConfig0Val(pub u8);
impl TempConfig0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn temp_filt_bw<'a>(&'a mut self) -> TempFiltBw<'a> {
        TempFiltBw(self)
    }
}
pub struct TempFiltBw<'a>(pub &'a mut TempConfig0Val);
impl<'a> TempFiltBw<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TempConfig0Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
}
