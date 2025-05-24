use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncEnA<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncEnA<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncEnAVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x4, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncEnAVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncEnAVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x4, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncEnAVal(val))
    }
    pub fn write(&mut self, val: EmbFuncEnAVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x4, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncEnAVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncEnAVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x4, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncEnAVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncEnAVal) -> EmbFuncEnAVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncEnAVal) -> EmbFuncEnAVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncEnAVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncEnAVal(0x0)).await
    }
}
pub struct EmbFuncEnAVal(pub u8);
impl EmbFuncEnAVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
        Self(0)
    }
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn mlc_before_fsm_en<'a>(&'a mut self) -> FieldMlcBeforeFsmEn<'a> {
        FieldMlcBeforeFsmEn(self)
    }
    pub fn sign_motion_en<'a>(&'a mut self) -> FieldSignMotionEn<'a> {
        FieldSignMotionEn(self)
    }
    pub fn tilt_en<'a>(&'a mut self) -> FieldTiltEn<'a> {
        FieldTiltEn(self)
    }
    pub fn pedo_en<'a>(&'a mut self) -> FieldPedoEn<'a> {
        FieldPedoEn(self)
    }
    pub fn sflp_game_en<'a>(&'a mut self) -> FieldSflpGameEn<'a> {
        FieldSflpGameEn(self)
    }
}
pub struct FieldMlcBeforeFsmEn<'a>(pub &'a mut EmbFuncEnAVal);
impl<'a> FieldMlcBeforeFsmEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldSignMotionEn<'a>(pub &'a mut EmbFuncEnAVal);
impl<'a> FieldSignMotionEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldTiltEn<'a>(pub &'a mut EmbFuncEnAVal);
impl<'a> FieldTiltEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldPedoEn<'a>(pub &'a mut EmbFuncEnAVal);
impl<'a> FieldPedoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldSflpGameEn<'a>(pub &'a mut EmbFuncEnAVal);
impl<'a> FieldSflpGameEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncEnAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncEnAVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
