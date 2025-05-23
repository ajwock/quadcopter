use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FifoDataOutZ<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FifoDataOutZ<'a, C> {
    pub fn read(&mut self) -> Result<FifoDataOutZVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x7d, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(FifoDataOutZVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoDataOutZVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x7d, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(FifoDataOutZVal(val))
    }
}
pub struct FifoDataOutZVal(pub u16);
impl FifoDataOutZVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
