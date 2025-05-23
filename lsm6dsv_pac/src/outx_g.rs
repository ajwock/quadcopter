use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct OutxG<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> OutxG<'a, C> {
    pub fn read(&mut self) -> Result<OutxGVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x22, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(OutxGVal(val))
    }
    pub async fn read_async(&mut self) -> Result<OutxGVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x22, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(OutxGVal(val))
    }
}
pub struct OutxGVal(pub u16);
impl OutxGVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
