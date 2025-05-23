use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FifoDataOutX<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FifoDataOutX<'a, C> {
    pub fn read(&mut self) -> Result<FifoDataOutXVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x79, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(FifoDataOutXVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoDataOutXVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x79, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(FifoDataOutXVal(val))
    }
}
pub struct FifoDataOutXVal(pub u16);
impl FifoDataOutXVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
