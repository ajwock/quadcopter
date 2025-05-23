use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct IfCfg<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> IfCfg<'a, C> {
    pub fn read(&mut self) -> Result<IfCfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x3, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(IfCfgVal(val))
    }
    pub async fn read_async(&mut self) -> Result<IfCfgVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x3, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(IfCfgVal(val))
    }
    pub fn write(&mut self, val: IfCfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x3, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IfCfgVal(raw_val))
    }
    pub async fn write_async(&mut self, val: IfCfgVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x3, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IfCfgVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(IfCfgVal) -> IfCfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IfCfgVal) -> IfCfgVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IfCfgVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IfCfgVal(0x0)).await
    }
}
pub struct IfCfgVal(pub u8);
impl IfCfgVal {
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
    pub fn sda_pu_en<'a>(&'a mut self) -> FieldSdaPuEn<'a> {
        FieldSdaPuEn(self)
    }
    pub fn shub_pu_en<'a>(&'a mut self) -> FieldShubPuEn<'a> {
        FieldShubPuEn(self)
    }
    pub fn asf_ctrl<'a>(&'a mut self) -> FieldAsfCtrl<'a> {
        FieldAsfCtrl(self)
    }
    pub fn h_lactive<'a>(&'a mut self) -> FieldHLactive<'a> {
        FieldHLactive(self)
    }
    pub fn pp_od<'a>(&'a mut self) -> FieldPpOd<'a> {
        FieldPpOd(self)
    }
    pub fn sim<'a>(&'a mut self) -> FieldSim<'a> {
        FieldSim(self)
    }
    pub fn i2_c_i3_c_disable<'a>(&'a mut self) -> FieldI2CI3CDisable<'a> {
        FieldI2CI3CDisable(self)
    }
}
pub struct FieldSdaPuEn<'a>(pub &'a mut IfCfgVal);
impl<'a> FieldSdaPuEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IfCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IfCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldShubPuEn<'a>(pub &'a mut IfCfgVal);
impl<'a> FieldShubPuEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IfCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IfCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldAsfCtrl<'a>(pub &'a mut IfCfgVal);
impl<'a> FieldAsfCtrl<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IfCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IfCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldHLactive<'a>(pub &'a mut IfCfgVal);
impl<'a> FieldHLactive<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IfCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IfCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldPpOd<'a>(pub &'a mut IfCfgVal);
impl<'a> FieldPpOd<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IfCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IfCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldSim<'a>(pub &'a mut IfCfgVal);
impl<'a> FieldSim<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IfCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IfCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldI2CI3CDisable<'a>(pub &'a mut IfCfgVal);
impl<'a> FieldI2CI3CDisable<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IfCfgVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IfCfgVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IfCfgVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
