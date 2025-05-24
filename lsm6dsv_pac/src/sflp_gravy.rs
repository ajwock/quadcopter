use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct SflpGravy<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> SflpGravy<'a, C> {
    pub fn read(&mut self) -> Result<SflpGravyVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x20, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpGravyVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SflpGravyVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x20, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpGravyVal(val))
    }
}
pub struct SflpGravyVal(pub u16);
impl SflpGravyVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
