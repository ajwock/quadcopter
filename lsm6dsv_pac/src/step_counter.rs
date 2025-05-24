use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct StepCounter<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> StepCounter<'a, C> {
    pub fn read(&mut self) -> Result<StepCounterVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x62, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(StepCounterVal(val))
    }
    pub async fn read_async(&mut self) -> Result<StepCounterVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x62, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(StepCounterVal(val))
    }
}
pub struct StepCounterVal(pub u16);
impl StepCounterVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
