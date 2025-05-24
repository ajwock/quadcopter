use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Mlc5Src<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Mlc5Src<'a, C> {
    pub fn read(&mut self) -> Result<Mlc5SrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x74, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Mlc5SrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Mlc5SrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x74, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Mlc5SrcVal(val))
    }
}
pub struct Mlc5SrcVal(pub u8);
impl Mlc5SrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
}
