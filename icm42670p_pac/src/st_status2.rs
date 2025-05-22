use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct StStatus2<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> StStatus2<'a, C> {
    pub fn read(&mut self) -> Result<StStatus2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x64, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(StStatus2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<StStatus2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x64, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(StStatus2Val(val))
    }
}
pub struct StStatus2Val(pub u8);
impl StStatus2Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn st_incomplete<'a>(&'a mut self) -> FieldStIncomplete<'a> {
        FieldStIncomplete(self)
    }
    pub fn gyro_st_pass<'a>(&'a mut self) -> FieldGyroStPass<'a> {
        FieldGyroStPass(self)
    }
    pub fn gyro_st_done<'a>(&'a mut self) -> FieldGyroStDone<'a> {
        FieldGyroStDone(self)
    }
    pub fn gz_st_pass<'a>(&'a mut self) -> FieldGzStPass<'a> {
        FieldGzStPass(self)
    }
    pub fn gy_st_pass<'a>(&'a mut self) -> FieldGyStPass<'a> {
        FieldGyStPass(self)
    }
    pub fn gx_st_pass<'a>(&'a mut self) -> FieldGxStPass<'a> {
        FieldGxStPass(self)
    }
}
pub struct FieldStIncomplete<'a>(pub &'a mut StStatus2Val);
impl<'a> FieldStIncomplete<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldGyroStPass<'a>(pub &'a mut StStatus2Val);
impl<'a> FieldGyroStPass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldGyroStDone<'a>(pub &'a mut StStatus2Val);
impl<'a> FieldGyroStDone<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldGzStPass<'a>(pub &'a mut StStatus2Val);
impl<'a> FieldGzStPass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldGyStPass<'a>(pub &'a mut StStatus2Val);
impl<'a> FieldGyStPass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldGxStPass<'a>(pub &'a mut StStatus2Val);
impl<'a> FieldGxStPass<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
