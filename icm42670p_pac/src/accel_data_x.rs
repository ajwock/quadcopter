use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct AccelDataX<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> AccelDataX<'a, D, C> {
    pub fn read(&mut self) -> Result<AccelDataXVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0xb, &mut buf)?;
        let val = u16::from_be_bytes(buf);
        Ok(AccelDataXVal(val))
    }
    pub async fn read_async(&mut self) -> Result<AccelDataXVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0xb, &mut buf).await?;
        let val = u16::from_be_bytes(buf);
        Ok(AccelDataXVal(val))
    }
}
pub struct AccelDataXVal(pub u16);
impl AccelDataXVal {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x80)
    }
}
