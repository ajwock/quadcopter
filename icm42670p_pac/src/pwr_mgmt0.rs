use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct PwrMgmt0<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> PwrMgmt0<'a, D, C> {
    pub fn read(&mut self) -> Result<PwrMgmt0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1f, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(PwrMgmt0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<PwrMgmt0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1f, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(PwrMgmt0Val(val))
    }
    pub fn write(&mut self, val: PwrMgmt0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x1f, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(PwrMgmt0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: PwrMgmt0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x1f, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(PwrMgmt0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(PwrMgmt0Val) -> PwrMgmt0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(PwrMgmt0Val) -> PwrMgmt0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(PwrMgmt0Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(PwrMgmt0Val(0x0)).await
    }
}
pub struct PwrMgmt0Val(pub u8);
impl PwrMgmt0Val {
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
    pub fn accel_lp_clk_sel<'a>(&'a mut self) -> FieldAccelLpClkSel<'a> {
        FieldAccelLpClkSel(self)
    }
    pub fn idle<'a>(&'a mut self) -> FieldIdle<'a> {
        FieldIdle(self)
    }
    pub fn gyro_mode<'a>(&'a mut self) -> FieldGyroMode<'a> {
        FieldGyroMode(self)
    }
    pub fn accel_mode<'a>(&'a mut self) -> FieldAccelMode<'a> {
        FieldAccelMode(self)
    }
}
pub struct FieldAccelLpClkSel<'a>(pub &'a mut PwrMgmt0Val);
impl<'a> FieldAccelLpClkSel<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut PwrMgmt0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PwrMgmt0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldIdle<'a>(pub &'a mut PwrMgmt0Val);
impl<'a> FieldIdle<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut PwrMgmt0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PwrMgmt0Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldGyroMode<'a>(pub &'a mut PwrMgmt0Val);
impl<'a> FieldGyroMode<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 2;
        self.0
    }
    pub fn reset(self) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= 0x0 & (!(!0 << 2) << 2);
        self.0
    }
}
pub struct FieldAccelMode<'a>(pub &'a mut PwrMgmt0Val);
impl<'a> FieldAccelMode<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x0 & (!(!0 << 2) << 0);
        self.0
    }
}
