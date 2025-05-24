use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncExecStatus<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncExecStatus<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncExecStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x7, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncExecStatusVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncExecStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x7, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncExecStatusVal(val))
    }
}
pub struct EmbFuncExecStatusVal(pub u8);
impl EmbFuncExecStatusVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn emb_func_exec_ovr<'a>(&'a mut self) -> FieldEmbFuncExecOvr<'a> {
        FieldEmbFuncExecOvr(self)
    }
    pub fn emb_func_endop<'a>(&'a mut self) -> FieldEmbFuncEndop<'a> {
        FieldEmbFuncEndop(self)
    }
}
pub struct FieldEmbFuncExecOvr<'a>(pub &'a mut EmbFuncExecStatusVal);
impl<'a> FieldEmbFuncExecOvr<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldEmbFuncEndop<'a>(pub &'a mut EmbFuncExecStatusVal);
impl<'a> FieldEmbFuncEndop<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
