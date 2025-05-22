use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig4<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexConfig4<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexConfig4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x46, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig4Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig4Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x46, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig4Val(val))
    }
    pub fn write(&mut self, val: ApexConfig4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x46, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig4Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig4Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x46, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig4Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig4Val) -> ApexConfig4Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig4Val) -> ApexConfig4Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig4Val(0x51))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig4Val(0x51)).await
    }
}
pub struct ApexConfig4Val(pub u8);
impl ApexConfig4Val {
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
        Self(0x51)
    }
    pub fn ped_step_det_th_sel<'a>(&'a mut self) -> FieldPedStepDetThSel<'a> {
        FieldPedStepDetThSel(self)
    }
    pub fn ped_sb_timer_th_sel<'a>(&'a mut self) -> FieldPedSbTimerThSel<'a> {
        FieldPedSbTimerThSel(self)
    }
    pub fn ped_hi_en_th_sel<'a>(&'a mut self) -> FieldPedHiEnThSel<'a> {
        FieldPedHiEnThSel(self)
    }
}
pub struct FieldPedStepDetThSel<'a>(pub &'a mut ApexConfig4Val);
impl<'a> FieldPedStepDetThSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig4Val {
        self.0.0 &= !(!(!0 << 3) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 5;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig4Val {
        self.0.0 &= !(!(!0 << 3) << 5);
        self.0.0 |= 0x51 & (!(!0 << 3) << 5);
        self.0
    }
}
pub struct FieldPedSbTimerThSel<'a>(pub &'a mut ApexConfig4Val);
impl<'a> FieldPedSbTimerThSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig4Val {
        self.0.0 &= !(!(!0 << 3) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 2;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig4Val {
        self.0.0 &= !(!(!0 << 3) << 2);
        self.0.0 |= 0x51 & (!(!0 << 3) << 2);
        self.0
    }
}
pub struct FieldPedHiEnThSel<'a>(pub &'a mut ApexConfig4Val);
impl<'a> FieldPedHiEnThSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig4Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig4Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x51 & (!(!0 << 2) << 0);
        self.0
    }
}
