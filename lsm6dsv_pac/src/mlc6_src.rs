use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Mlc6Src<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Mlc6Src<'a, C> {
    pub fn read(&mut self) -> Result<Mlc6SrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x75, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Mlc6SrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Mlc6SrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x75, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Mlc6SrcVal(val))
    }
}
pub struct Mlc6SrcVal(pub u8);
impl Mlc6SrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
}
