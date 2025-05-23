use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FifoStatus1<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FifoStatus1<'a, C> {
    pub fn read(&mut self) -> Result<FifoStatus1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1b, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoStatus1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoStatus1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1b, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoStatus1Val(val))
    }
}
pub struct FifoStatus1Val(pub u8);
impl FifoStatus1Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn diff_fifo_7_0<'a>(&'a mut self) -> FieldDiffFifo70<'a> {
        FieldDiffFifo70(self)
    }
}
pub struct FieldDiffFifo70<'a>(pub &'a mut FifoStatus1Val);
impl<'a> FieldDiffFifo70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
