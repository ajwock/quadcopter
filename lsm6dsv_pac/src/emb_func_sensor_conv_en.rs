use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct EmbFuncSensorConvEn<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> EmbFuncSensorConvEn<'a, C> {
    pub fn read(&mut self) -> Result<EmbFuncSensorConvEnVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x6e, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncSensorConvEnVal(val))
    }
    pub async fn read_async(&mut self) -> Result<EmbFuncSensorConvEnVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x6e, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(EmbFuncSensorConvEnVal(val))
    }
    pub fn write(&mut self, val: EmbFuncSensorConvEnVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x6e, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(EmbFuncSensorConvEnVal(raw_val))
    }
    pub async fn write_async(&mut self, val: EmbFuncSensorConvEnVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x6e, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncSensorConvEnVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(EmbFuncSensorConvEnVal) -> EmbFuncSensorConvEnVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(EmbFuncSensorConvEnVal) -> EmbFuncSensorConvEnVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(EmbFuncSensorConvEnVal(0xf))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(EmbFuncSensorConvEnVal(0xf)).await
    }
}
pub struct EmbFuncSensorConvEnVal(pub u8);
impl EmbFuncSensorConvEnVal {
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
        Self(0xf)
    }
    pub fn ext_sensor_conv_en<'a>(&'a mut self) -> FieldExtSensorConvEn<'a> {
        FieldExtSensorConvEn(self)
    }
    pub fn temp_conv_en<'a>(&'a mut self) -> FieldTempConvEn<'a> {
        FieldTempConvEn(self)
    }
    pub fn gyro_conv_en<'a>(&'a mut self) -> FieldGyroConvEn<'a> {
        FieldGyroConvEn(self)
    }
    pub fn xl_hg_conv_en<'a>(&'a mut self) -> FieldXlHgConvEn<'a> {
        FieldXlHgConvEn(self)
    }
}
pub struct FieldExtSensorConvEn<'a>(pub &'a mut EmbFuncSensorConvEnVal);
impl<'a> FieldExtSensorConvEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSensorConvEnVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0xf;
        self.0
    }
}
pub struct FieldTempConvEn<'a>(pub &'a mut EmbFuncSensorConvEnVal);
impl<'a> FieldTempConvEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSensorConvEnVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0xf;
        self.0
    }
}
pub struct FieldGyroConvEn<'a>(pub &'a mut EmbFuncSensorConvEnVal);
impl<'a> FieldGyroConvEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSensorConvEnVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0xf;
        self.0
    }
}
pub struct FieldXlHgConvEn<'a>(pub &'a mut EmbFuncSensorConvEnVal);
impl<'a> FieldXlHgConvEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut EmbFuncSensorConvEnVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut EmbFuncSensorConvEnVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0xf;
        self.0
    }
}
