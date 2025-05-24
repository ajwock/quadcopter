use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct SflpGbiasy<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> SflpGbiasy<'a, C> {
    pub fn read(&mut self) -> Result<SflpGbiasyVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x1a, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpGbiasyVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SflpGbiasyVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x1a, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpGbiasyVal(val))
    }
}
pub struct SflpGbiasyVal(pub u16);
impl SflpGbiasyVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
