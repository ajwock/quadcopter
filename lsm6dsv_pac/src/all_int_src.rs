use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct AllIntSrc<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> AllIntSrc<'a, C> {
    pub fn read(&mut self) -> Result<AllIntSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1d, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(AllIntSrcVal(val))
    }
    pub async fn read_async(&mut self) -> Result<AllIntSrcVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1d, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(AllIntSrcVal(val))
    }
}
pub struct AllIntSrcVal(pub u8);
impl AllIntSrcVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn emb_func_ia<'a>(&'a mut self) -> FieldEmbFuncIa<'a> {
        FieldEmbFuncIa(self)
    }
    pub fn shub_ia<'a>(&'a mut self) -> FieldShubIa<'a> {
        FieldShubIa(self)
    }
    pub fn sleep_change_ia<'a>(&'a mut self) -> FieldSleepChangeIa<'a> {
        FieldSleepChangeIa(self)
    }
    pub fn d6_d_ia<'a>(&'a mut self) -> FieldD6DIa<'a> {
        FieldD6DIa(self)
    }
    pub fn hg_ia<'a>(&'a mut self) -> FieldHgIa<'a> {
        FieldHgIa(self)
    }
    pub fn tap_ia<'a>(&'a mut self) -> FieldTapIa<'a> {
        FieldTapIa(self)
    }
    pub fn wu_ia<'a>(&'a mut self) -> FieldWuIa<'a> {
        FieldWuIa(self)
    }
    pub fn ff_ia<'a>(&'a mut self) -> FieldFfIa<'a> {
        FieldFfIa(self)
    }
}
pub struct FieldEmbFuncIa<'a>(pub &'a mut AllIntSrcVal);
impl<'a> FieldEmbFuncIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldShubIa<'a>(pub &'a mut AllIntSrcVal);
impl<'a> FieldShubIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldSleepChangeIa<'a>(pub &'a mut AllIntSrcVal);
impl<'a> FieldSleepChangeIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldD6DIa<'a>(pub &'a mut AllIntSrcVal);
impl<'a> FieldD6DIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldHgIa<'a>(pub &'a mut AllIntSrcVal);
impl<'a> FieldHgIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldTapIa<'a>(pub &'a mut AllIntSrcVal);
impl<'a> FieldTapIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldWuIa<'a>(pub &'a mut AllIntSrcVal);
impl<'a> FieldWuIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFfIa<'a>(pub &'a mut AllIntSrcVal);
impl<'a> FieldFfIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
