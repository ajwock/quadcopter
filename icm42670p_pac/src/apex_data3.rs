use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexData3<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> ApexData3<'a, C> {
    pub fn read(&mut self) -> Result<ApexData3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x34, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexData3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x34, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexData3Val(val))
    }
}
pub struct ApexData3Val(pub u8);
impl ApexData3Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x4)
    }
    pub fn dmp_idle<'a>(&'a mut self) -> FieldDmpIdle<'a> {
        FieldDmpIdle(self)
    }
    pub fn activity_class<'a>(&'a mut self) -> FieldActivityClass<'a> {
        FieldActivityClass(self)
    }
}
pub struct FieldDmpIdle<'a>(pub &'a mut ApexData3Val);
impl<'a> FieldDmpIdle<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldActivityClass<'a>(pub &'a mut ApexData3Val);
impl<'a> FieldActivityClass<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
}
