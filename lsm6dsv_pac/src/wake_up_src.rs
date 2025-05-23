use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct WakeUpSrc<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> WakeUpSrc<'a, C> {
    pub fn read(&mut self) -> Result<WakeUpSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x45, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(WakeUpSrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<WakeUpSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x45, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(WakeUpSrcVal(val))
    }
}
pub struct WakeUpSrcVal(pub u8);
impl WakeUpSrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn sleep_change_ia<'a>(&'a mut self) -> FieldSleepChangeIa<'a> {
        FieldSleepChangeIa(self)
    }
    pub fn ff_ia<'a>(&'a mut self) -> FieldFfIa<'a> {
        FieldFfIa(self)
    }
    pub fn sleep_state<'a>(&'a mut self) -> FieldSleepState<'a> {
        FieldSleepState(self)
    }
    pub fn wu_ia<'a>(&'a mut self) -> FieldWuIa<'a> {
        FieldWuIa(self)
    }
    pub fn x_wu<'a>(&'a mut self) -> FieldXWu<'a> {
        FieldXWu(self)
    }
    pub fn y_wu<'a>(&'a mut self) -> FieldYWu<'a> {
        FieldYWu(self)
    }
    pub fn z_wu<'a>(&'a mut self) -> FieldZWu<'a> {
        FieldZWu(self)
    }
}
pub struct FieldSleepChangeIa<'a>(pub &'a mut WakeUpSrcVal);
impl<'a> FieldSleepChangeIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFfIa<'a>(pub &'a mut WakeUpSrcVal);
impl<'a> FieldFfIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldSleepState<'a>(pub &'a mut WakeUpSrcVal);
impl<'a> FieldSleepState<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldWuIa<'a>(pub &'a mut WakeUpSrcVal);
impl<'a> FieldWuIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldXWu<'a>(pub &'a mut WakeUpSrcVal);
impl<'a> FieldXWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldYWu<'a>(pub &'a mut WakeUpSrcVal);
impl<'a> FieldYWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldZWu<'a>(pub &'a mut WakeUpSrcVal);
impl<'a> FieldZWu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
