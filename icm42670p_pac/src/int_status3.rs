use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntStatus3<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntStatus3<'a, C> {
    pub fn read(&mut self) -> Result<IntStatus3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x3c, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatus3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntStatus3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x3c, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatus3Val(val))
    }
}
pub struct IntStatus3Val(pub u8);
impl IntStatus3Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn step_det_int<'a>(&'a mut self) -> StepDetInt<'a> {
        StepDetInt(self)
    }
    pub fn step_cnt_ovf_int<'a>(&'a mut self) -> StepCntOvfInt<'a> {
        StepCntOvfInt(self)
    }
    pub fn tilt_det_int<'a>(&'a mut self) -> TiltDetInt<'a> {
        TiltDetInt(self)
    }
    pub fn ff_det_int<'a>(&'a mut self) -> FfDetInt<'a> {
        FfDetInt(self)
    }
    pub fn lowg_det_int<'a>(&'a mut self) -> LowgDetInt<'a> {
        LowgDetInt(self)
    }
}
pub struct StepDetInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> StepDetInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct StepCntOvfInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> StepCntOvfInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct TiltDetInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> TiltDetInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FfDetInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> FfDetInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct LowgDetInt<'a>(pub &'a mut IntStatus3Val);
impl<'a> LowgDetInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
