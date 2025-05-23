use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct CtrlStatus<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> CtrlStatus<'a, C> {
    pub fn read(&mut self) -> Result<CtrlStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1a, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(CtrlStatusVal(val))
    }
    pub async fn read_async(&mut self) -> Result<CtrlStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1a, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(CtrlStatusVal(val))
    }
}
pub struct CtrlStatusVal(pub u8);
impl CtrlStatusVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn fsm_wr_ctrl_status<'a>(&'a mut self) -> FieldFsmWrCtrlStatus<'a> {
        FieldFsmWrCtrlStatus(self)
    }
}
pub struct FieldFsmWrCtrlStatus<'a>(pub &'a mut CtrlStatusVal);
impl<'a> FieldFsmWrCtrlStatus<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
