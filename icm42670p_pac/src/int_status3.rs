use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntStatus3<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntStatus3<'a, C> {
    pub fn read(&mut self) -> Result<IntStatus3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x3c, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatus3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntStatus3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x3c, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatus3Val(val))
    }
}
pub struct IntStatus3Val(pub u8);
impl IntStatus3Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn step_det_int<'a>(&'a mut self) -> FieldStepDetInt<'a> {
        FieldStepDetInt(self)
    }
    pub fn step_cnt_ovf_int<'a>(&'a mut self) -> FieldStepCntOvfInt<'a> {
        FieldStepCntOvfInt(self)
    }
    pub fn tilt_det_int<'a>(&'a mut self) -> FieldTiltDetInt<'a> {
        FieldTiltDetInt(self)
    }
    pub fn ff_det_int<'a>(&'a mut self) -> FieldFfDetInt<'a> {
        FieldFfDetInt(self)
    }
    pub fn lowg_det_int<'a>(&'a mut self) -> FieldLowgDetInt<'a> {
        FieldLowgDetInt(self)
    }
}
pub struct FieldStepDetInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> FieldStepDetInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldStepCntOvfInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> FieldStepCntOvfInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldTiltDetInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> FieldTiltDetInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFfDetInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> FieldFfDetInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldLowgDetInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> FieldLowgDetInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
