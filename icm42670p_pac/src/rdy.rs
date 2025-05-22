use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct Rdy<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> Rdy<'a, D, C> {
    pub fn read(&mut self) -> Result<RdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x0, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(RdyVal(val))
    }
    pub async fn read_async(&mut self) -> Result<RdyVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x0, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(RdyVal(val))
    }
}
pub struct RdyVal(pub u8);
impl RdyVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn mclk_rdy<'a>(&'a mut self) -> FieldMclkRdy<'a> {
        FieldMclkRdy(self)
    }
}
pub struct FieldMclkRdy<'a>(pub &'a mut RdyVal);
impl<'a> FieldMclkRdy<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
