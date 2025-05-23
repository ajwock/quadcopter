use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Timestamp<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Timestamp<'a, C> {
    pub fn read(&mut self) -> Result<TimestampVal, RegCommsError> {
        let mut buf = [0u8; 4];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x40, &mut buf)?;
        let val = u32::from_le_bytes(buf);
        Ok(TimestampVal(val))
    }
    pub async fn read_async(&mut self) -> Result<TimestampVal, RegCommsError> {
        let mut buf = [0u8; 4];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x40, &mut buf).await?;
        let val = u32::from_le_bytes(buf);
        Ok(TimestampVal(val))
    }
}
pub struct TimestampVal(pub u32);
impl TimestampVal {
    pub fn get(&self) -> u32 {
        self.0
    }
}
