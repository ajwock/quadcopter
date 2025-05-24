use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct SflpQuatx<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> SflpQuatx<'a, C> {
    pub fn read(&mut self) -> Result<SflpQuatxVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x2c, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpQuatxVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SflpQuatxVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x2c, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpQuatxVal(val))
    }
}
pub struct SflpQuatxVal(pub u16);
impl SflpQuatxVal {
    pub fn get(&self) -> u16 {
        self.0
    }
}
