use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncInitA<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncInitA<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncInitAVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x66, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncInitAVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncInitAVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x66, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncInitAVal(val))
    }
    pub fn write(&mut self, val: EmbFuncInitAVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x66, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncInitAVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncInitAVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x66, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncInitAVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncInitAVal) -> EmbFuncInitAVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncInitAVal) -> EmbFuncInitAVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncInitAVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncInitAVal(0x0)).await
    }
}
pub struct EmbFuncInitAVal(pub u8);
impl EmbFuncInitAVal {
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
    pub fn mlc_before_fsm_init<'a>(&'a mut self) -> FieldMlcBeforeFsmInit<'a> {
        FieldMlcBeforeFsmInit(self)
    }
    pub fn sig_mot_init<'a>(&'a mut self) -> FieldSigMotInit<'a> {
        FieldSigMotInit(self)
    }
    pub fn tilt_init<'a>(&'a mut self) -> FieldTiltInit<'a> {
        FieldTiltInit(self)
    }
    pub fn step_det_init<'a>(&'a mut self) -> FieldStepDetInit<'a> {
        FieldStepDetInit(self)
    }
    pub fn sflp_game_init<'a>(&'a mut self) -> FieldSflpGameInit<'a> {
        FieldSflpGameInit(self)
    }
}
pub struct FieldMlcBeforeFsmInit<'a>(pub &'a mut EmbFuncInitAVal);
impl<'a> FieldMlcBeforeFsmInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldSigMotInit<'a>(pub &'a mut EmbFuncInitAVal);
impl<'a> FieldSigMotInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldTiltInit<'a>(pub &'a mut EmbFuncInitAVal);
impl<'a> FieldTiltInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldStepDetInit<'a>(pub &'a mut EmbFuncInitAVal);
impl<'a> FieldStepDetInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldSflpGameInit<'a>(pub &'a mut EmbFuncInitAVal);
impl<'a> FieldSflpGameInit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncInitAVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncInitAVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
