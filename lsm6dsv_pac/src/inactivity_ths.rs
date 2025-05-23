use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct InactivityThs<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> InactivityThs<'a, C> {
    pub fn read(&mut self) -> Result<InactivityThsVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x55, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(InactivityThsVal(val))
    }
    pub async fn read_async(&mut self) -> Result<InactivityThsVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x55, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(InactivityThsVal(val))
    }
    pub fn write(&mut self, val: InactivityThsVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x55, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(InactivityThsVal(raw_val))
    }
    pub async fn write_async(&mut self, val: InactivityThsVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x55, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(InactivityThsVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(InactivityThsVal) -> InactivityThsVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(InactivityThsVal) -> InactivityThsVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(InactivityThsVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(InactivityThsVal(0x0)).await
    }
}
pub struct InactivityThsVal(pub u8);
impl InactivityThsVal {
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
    pub fn int2_hg_shock_change<'a>(&'a mut self) -> FieldInt2HgShockChange<'a> {
        FieldInt2HgShockChange(self)
    }
    pub fn int1_hg_shock_change<'a>(&'a mut self) -> FieldInt1HgShockChange<'a> {
        FieldInt1HgShockChange(self)
    }
    pub fn inact_ths<'a>(&'a mut self) -> FieldInactThs<'a> {
        FieldInactThs(self)
    }
}
pub struct FieldInt2HgShockChange<'a>(pub &'a mut InactivityThsVal);
impl<'a> FieldInt2HgShockChange<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut InactivityThsVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut InactivityThsVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut InactivityThsVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut InactivityThsVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldInt1HgShockChange<'a>(pub &'a mut InactivityThsVal);
impl<'a> FieldInt1HgShockChange<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut InactivityThsVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut InactivityThsVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut InactivityThsVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut InactivityThsVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldInactThs<'a>(pub &'a mut InactivityThsVal);
impl<'a> FieldInactThs<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 6)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut InactivityThsVal {
        self.0.0 &= !(!(!0 << 6) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 6)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut InactivityThsVal {
        self.0.0 &= !(!(!0 << 6) << 0);
        self.0.0 |= 0x0 & (!(!0 << 6) << 0);
        self.0
    }
}
