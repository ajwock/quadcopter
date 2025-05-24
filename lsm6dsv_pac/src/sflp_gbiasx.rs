use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct SflpGbiasx<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> SflpGbiasx<'a, C> {
    pub fn read(&mut self) -> Result<SflpGbiasxVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x18, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpGbiasxVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SflpGbiasxVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x18, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpGbiasxVal(val))
    }
}
pub struct SflpGbiasxVal(pub u16);
impl SflpGbiasxVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
