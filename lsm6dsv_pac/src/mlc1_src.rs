use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Mlc1Src<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Mlc1Src<'a, C> {
    pub fn read(&mut self) -> Result<Mlc1SrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x70, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Mlc1SrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Mlc1SrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x70, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Mlc1SrcVal(val))
    }
}
pub struct Mlc1SrcVal(pub u8);
impl Mlc1SrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
}
