use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FsmEnable<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FsmEnable<'a, C> {
    pub fn read(&mut self) -> Result<FsmEnableVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x46, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FsmEnableVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FsmEnableVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x46, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FsmEnableVal(val))
    }
    pub fn write(&mut self, val: FsmEnableVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x46, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FsmEnableVal(raw_val))
    }
    pub async fn write_async(&mut self, val: FsmEnableVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x46, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FsmEnableVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(FsmEnableVal) -> FsmEnableVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FsmEnableVal) -> FsmEnableVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FsmEnableVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FsmEnableVal(0x0)).await
    }
}
pub struct FsmEnableVal(pub u8);
impl FsmEnableVal {
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
    pub fn fsm8_en<'a>(&'a mut self) -> FieldFsm8En<'a> {
        FieldFsm8En(self)
    }
    pub fn fsm7_en<'a>(&'a mut self) -> FieldFsm7En<'a> {
        FieldFsm7En(self)
    }
    pub fn fsm6_en<'a>(&'a mut self) -> FieldFsm6En<'a> {
        FieldFsm6En(self)
    }
    pub fn fsm5_en<'a>(&'a mut self) -> FieldFsm5En<'a> {
        FieldFsm5En(self)
    }
    pub fn fsm4_en<'a>(&'a mut self) -> FieldFsm4En<'a> {
        FieldFsm4En(self)
    }
    pub fn fsm3_en<'a>(&'a mut self) -> FieldFsm3En<'a> {
        FieldFsm3En(self)
    }
    pub fn fsm2_en<'a>(&'a mut self) -> FieldFsm2En<'a> {
        FieldFsm2En(self)
    }
    pub fn fsm1_en<'a>(&'a mut self) -> FieldFsm1En<'a> {
        FieldFsm1En(self)
    }
}
pub struct FieldFsm8En<'a>(pub &'a mut FsmEnableVal);
impl<'a> FieldFsm8En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldFsm7En<'a>(pub &'a mut FsmEnableVal);
impl<'a> FieldFsm7En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldFsm6En<'a>(pub &'a mut FsmEnableVal);
impl<'a> FieldFsm6En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldFsm5En<'a>(pub &'a mut FsmEnableVal);
impl<'a> FieldFsm5En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldFsm4En<'a>(pub &'a mut FsmEnableVal);
impl<'a> FieldFsm4En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldFsm3En<'a>(pub &'a mut FsmEnableVal);
impl<'a> FieldFsm3En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldFsm2En<'a>(pub &'a mut FsmEnableVal);
impl<'a> FieldFsm2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldFsm1En<'a>(pub &'a mut FsmEnableVal);
impl<'a> FieldFsm1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmEnableVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmEnableVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmEnableVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
