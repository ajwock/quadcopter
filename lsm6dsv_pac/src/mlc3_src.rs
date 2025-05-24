use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Mlc3Src<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Mlc3Src<'a, C> {
    pub fn read(&mut self) -> Result<Mlc3SrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x72, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Mlc3SrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<Mlc3SrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x72, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Mlc3SrcVal(val))
    }
}
pub struct Mlc3SrcVal(pub u8);
impl Mlc3SrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
}
