use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct StStatus1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> StStatus1<'a, C> {
    pub fn read(&mut self) -> Result<StStatus1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x63, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(StStatus1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<StStatus1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x63, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(StStatus1Val(val))
    }
}
pub struct StStatus1Val(pub u8);
impl StStatus1Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn accel_st_pass<'a>(&'a mut self) -> FieldAccelStPass<'a> {
        FieldAccelStPass(self)
    }
    pub fn accel_st_done<'a>(&'a mut self) -> FieldAccelStDone<'a> {
        FieldAccelStDone(self)
    }
    pub fn az_st_pass<'a>(&'a mut self) -> FieldAzStPass<'a> {
        FieldAzStPass(self)
    }
    pub fn ay_st_pass<'a>(&'a mut self) -> FieldAyStPass<'a> {
        FieldAyStPass(self)
    }
    pub fn ax_st_pass<'a>(&'a mut self) -> FieldAxStPass<'a> {
        FieldAxStPass(self)
    }
}
pub struct FieldAccelStPass<'a>(pub &'a mut StStatus1Val);
impl<'a> FieldAccelStPass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldAccelStDone<'a>(pub &'a mut StStatus1Val);
impl<'a> FieldAccelStDone<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldAzStPass<'a>(pub &'a mut StStatus1Val);
impl<'a> FieldAzStPass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldAyStPass<'a>(pub &'a mut StStatus1Val);
impl<'a> FieldAyStPass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldAxStPass<'a>(pub &'a mut StStatus1Val);
impl<'a> FieldAxStPass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
