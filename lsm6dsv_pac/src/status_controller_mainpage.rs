use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct StatusControllerMainpage<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> StatusControllerMainpage<'a, C> {
    pub fn read(&mut self) -> Result<StatusControllerMainpageVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x48, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(StatusControllerMainpageVal(val))
    }
    pub async fn read_async(&mut self) -> Result<StatusControllerMainpageVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x48, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(StatusControllerMainpageVal(val))
    }
}
pub struct StatusControllerMainpageVal(pub u8);
impl StatusControllerMainpageVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn wr_once_done<'a>(&'a mut self) -> FieldWrOnceDone<'a> {
        FieldWrOnceDone(self)
    }
    pub fn target3_nack<'a>(&'a mut self) -> FieldTarget3Nack<'a> {
        FieldTarget3Nack(self)
    }
    pub fn target2_nack<'a>(&'a mut self) -> FieldTarget2Nack<'a> {
        FieldTarget2Nack(self)
    }
    pub fn target1_nack<'a>(&'a mut self) -> FieldTarget1Nack<'a> {
        FieldTarget1Nack(self)
    }
    pub fn target0_nack<'a>(&'a mut self) -> FieldTarget0Nack<'a> {
        FieldTarget0Nack(self)
    }
    pub fn sens_hub_endop<'a>(&'a mut self) -> FieldSensHubEndop<'a> {
        FieldSensHubEndop(self)
    }
}
pub struct FieldWrOnceDone<'a>(pub &'a mut StatusControllerMainpageVal);
impl<'a> FieldWrOnceDone<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldTarget3Nack<'a>(pub &'a mut StatusControllerMainpageVal);
impl<'a> FieldTarget3Nack<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldTarget2Nack<'a>(pub &'a mut StatusControllerMainpageVal);
impl<'a> FieldTarget2Nack<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldTarget1Nack<'a>(pub &'a mut StatusControllerMainpageVal);
impl<'a> FieldTarget1Nack<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldTarget0Nack<'a>(pub &'a mut StatusControllerMainpageVal);
impl<'a> FieldTarget0Nack<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldSensHubEndop<'a>(pub &'a mut StatusControllerMainpageVal);
impl<'a> FieldSensHubEndop<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
