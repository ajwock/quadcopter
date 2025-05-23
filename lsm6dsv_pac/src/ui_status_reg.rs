use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct UiStatusReg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> UiStatusReg<'a, C> {
    pub fn read(&mut self) -> Result<UiStatusRegVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x44, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(UiStatusRegVal(val))
    }
    pub async fn read_async(&mut self) -> Result<UiStatusRegVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x44, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(UiStatusRegVal(val))
    }
}
pub struct UiStatusRegVal(pub u8);
impl UiStatusRegVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn gyro_settling<'a>(&'a mut self) -> FieldGyroSettling<'a> {
        FieldGyroSettling(self)
    }
}
pub struct FieldGyroSettling<'a>(pub &'a mut UiStatusRegVal);
impl<'a> FieldGyroSettling<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
