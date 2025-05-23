use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct PinCtrl<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> PinCtrl<'a, C> {
    pub fn read(&mut self) -> Result<PinCtrlVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x2, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(PinCtrlVal(val))
    }
    pub async fn read_async(&mut self) -> Result<PinCtrlVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x2, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(PinCtrlVal(val))
    }
    pub fn write(&mut self, val: PinCtrlVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x2, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(PinCtrlVal(raw_val))
    }
    pub async fn write_async(&mut self, val: PinCtrlVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x2, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(PinCtrlVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(PinCtrlVal) -> PinCtrlVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(PinCtrlVal) -> PinCtrlVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(PinCtrlVal(0x23))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(PinCtrlVal(0x23)).await
    }
}
pub struct PinCtrlVal(pub u8);
impl PinCtrlVal {
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
        Self(0x23)
    }
    pub fn sdo_pu_en<'a>(&'a mut self) -> FieldSdoPuEn<'a> {
        FieldSdoPuEn(self)
    }
    pub fn ibhr_por_en<'a>(&'a mut self) -> FieldIbhrPorEn<'a> {
        FieldIbhrPorEn(self)
    }
    pub fn io_pad_strength<'a>(&'a mut self) -> FieldIoPadStrength<'a> {
        FieldIoPadStrength(self)
    }
}
pub struct FieldSdoPuEn<'a>(pub &'a mut PinCtrlVal);
impl<'a> FieldSdoPuEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PinCtrlVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut PinCtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PinCtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut PinCtrlVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x23;
        self.0
    }
}
pub struct FieldIbhrPorEn<'a>(pub &'a mut PinCtrlVal);
impl<'a> FieldIbhrPorEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PinCtrlVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut PinCtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PinCtrlVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut PinCtrlVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x23;
        self.0
    }
}
pub struct FieldIoPadStrength<'a>(pub &'a mut PinCtrlVal);
impl<'a> FieldIoPadStrength<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut PinCtrlVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut PinCtrlVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x23 & (!(!0 << 2) << 0);
        self.0
    }
}
