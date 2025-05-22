use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct AccelDataZ<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> AccelDataZ<'a, C> {
    pub fn read(&mut self) -> Result<AccelDataZVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0xf, &mut buf)?;
        let val = u16::from_be_bytes(buf);
        Ok(AccelDataZVal(val))
    }
    pub async fn read_async(&mut self) -> Result<AccelDataZVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0xf, &mut buf).await?;
        let val = u16::from_be_bytes(buf);
        Ok(AccelDataZVal(val))
    }
}
pub struct AccelDataZVal(pub u16);
impl AccelDataZVal {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x80)
    }
}
