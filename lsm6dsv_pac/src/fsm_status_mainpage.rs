use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FsmStatusMainpage<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FsmStatusMainpage<'a, C> {
    pub fn read(&mut self) -> Result<FsmStatusMainpageVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x4a, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FsmStatusMainpageVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FsmStatusMainpageVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x4a, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FsmStatusMainpageVal(val))
    }
}
pub struct FsmStatusMainpageVal(pub u8);
impl FsmStatusMainpageVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn is_fsm8<'a>(&'a mut self) -> FieldIsFsm8<'a> {
        FieldIsFsm8(self)
    }
    pub fn is_fsm7<'a>(&'a mut self) -> FieldIsFsm7<'a> {
        FieldIsFsm7(self)
    }
    pub fn is_fsm6<'a>(&'a mut self) -> FieldIsFsm6<'a> {
        FieldIsFsm6(self)
    }
    pub fn is_fsm5<'a>(&'a mut self) -> FieldIsFsm5<'a> {
        FieldIsFsm5(self)
    }
    pub fn is_fsm4<'a>(&'a mut self) -> FieldIsFsm4<'a> {
        FieldIsFsm4(self)
    }
    pub fn is_fsm3<'a>(&'a mut self) -> FieldIsFsm3<'a> {
        FieldIsFsm3(self)
    }
    pub fn is_fsm2<'a>(&'a mut self) -> FieldIsFsm2<'a> {
        FieldIsFsm2(self)
    }
    pub fn is_fsm1<'a>(&'a mut self) -> FieldIsFsm1<'a> {
        FieldIsFsm1(self)
    }
}
pub struct FieldIsFsm8<'a>(pub &'a mut FsmStatusMainpageVal);
impl<'a> FieldIsFsm8<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsFsm7<'a>(pub &'a mut FsmStatusMainpageVal);
impl<'a> FieldIsFsm7<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsFsm6<'a>(pub &'a mut FsmStatusMainpageVal);
impl<'a> FieldIsFsm6<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsFsm5<'a>(pub &'a mut FsmStatusMainpageVal);
impl<'a> FieldIsFsm5<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsFsm4<'a>(pub &'a mut FsmStatusMainpageVal);
impl<'a> FieldIsFsm4<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsFsm3<'a>(pub &'a mut FsmStatusMainpageVal);
impl<'a> FieldIsFsm3<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsFsm2<'a>(pub &'a mut FsmStatusMainpageVal);
impl<'a> FieldIsFsm2<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsFsm1<'a>(pub &'a mut FsmStatusMainpageVal);
impl<'a> FieldIsFsm1<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
