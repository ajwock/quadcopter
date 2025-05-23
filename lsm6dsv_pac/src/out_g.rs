use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct OutG<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> OutG<'a, C> {
    pub fn read(&mut self) -> Result<OutGVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x22, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(OutGVal(val))
    }
    pub async fn read_async(&mut self) -> Result<OutGVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x22, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(OutGVal(val))
    }
}
pub struct OutGVal(pub u16);
impl OutGVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
