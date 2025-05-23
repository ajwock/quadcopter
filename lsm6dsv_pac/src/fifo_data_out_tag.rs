use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FifoDataOutTag<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FifoDataOutTag<'a, C> {
    pub fn read(&mut self) -> Result<FifoDataOutTagVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x78, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoDataOutTagVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoDataOutTagVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x78, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoDataOutTagVal(val))
    }
}
pub struct FifoDataOutTagVal(pub u8);
impl FifoDataOutTagVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn tag_sensor<'a>(&'a mut self) -> FieldTagSensor<'a> {
        FieldTagSensor(self)
    }
    pub fn tag_cnt<'a>(&'a mut self) -> FieldTagCnt<'a> {
        FieldTagCnt(self)
    }
}
pub struct FieldTagSensor<'a>(pub &'a mut FifoDataOutTagVal);
impl<'a> FieldTagSensor<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 5)) as u8
    }
}
pub struct FieldTagCnt<'a>(pub &'a mut FifoDataOutTagVal);
impl<'a> FieldTagCnt<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 1) & !(!0 << 2)) as u8
    }
}
