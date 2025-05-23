use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct StatusReg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> StatusReg<'a, C> {
    pub fn read(&mut self) -> Result<StatusRegVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1e, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(StatusRegVal(val))
    }
    pub async fn read_async(&mut self) -> Result<StatusRegVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1e, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(StatusRegVal(val))
    }
}
pub struct StatusRegVal(pub u8);
impl StatusRegVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn timestamp_endcount<'a>(&'a mut self) -> FieldTimestampEndcount<'a> {
        FieldTimestampEndcount(self)
    }
    pub fn xlhgda<'a>(&'a mut self) -> FieldXlhgda<'a> {
        FieldXlhgda(self)
    }
    pub fn tda<'a>(&'a mut self) -> FieldTda<'a> {
        FieldTda(self)
    }
    pub fn gda<'a>(&'a mut self) -> FieldGda<'a> {
        FieldGda(self)
    }
    pub fn xlda<'a>(&'a mut self) -> FieldXlda<'a> {
        FieldXlda(self)
    }
}
pub struct FieldTimestampEndcount<'a>(pub &'a mut StatusRegVal);
impl<'a> FieldTimestampEndcount<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldXlhgda<'a>(pub &'a mut StatusRegVal);
impl<'a> FieldXlhgda<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldTda<'a>(pub &'a mut StatusRegVal);
impl<'a> FieldTda<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldGda<'a>(pub &'a mut StatusRegVal);
impl<'a> FieldGda<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldXlda<'a>(pub &'a mut StatusRegVal);
impl<'a> FieldXlda<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
