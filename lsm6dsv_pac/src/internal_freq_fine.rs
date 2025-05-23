use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct InternalFreqFine<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> InternalFreqFine<'a, C> {
    pub fn read(&mut self) -> Result<InternalFreqFineVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x4f, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(InternalFreqFineVal(val))
    }
    pub async fn read_async(&mut self) -> Result<InternalFreqFineVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x4f, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(InternalFreqFineVal(val))
    }
}
pub struct InternalFreqFineVal(pub u8);
impl InternalFreqFineVal {
    pub fn get(&self) -> u8 {
        self.0
    }
}
