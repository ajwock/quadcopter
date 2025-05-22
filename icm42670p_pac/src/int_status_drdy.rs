use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntStatusDrdy<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntStatusDrdy<'a, C> {
    pub fn read(&mut self) -> Result<IntStatusDrdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x39, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatusDrdyVal(val))
    }
    pub async fn read_async(&mut self) -> Result<IntStatusDrdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x39, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatusDrdyVal(val))
    }
}
pub struct IntStatusDrdyVal(pub u8);
impl IntStatusDrdyVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn data_rdy_int<'a>(&'a mut self) -> FieldDataRdyInt<'a> {
        FieldDataRdyInt(self)
    }
}
pub struct FieldDataRdyInt<'a>(pub &'a mut IntStatusDrdyVal);
impl<'a> FieldDataRdyInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
