use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct ApexConfig3<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> ApexConfig3<'a, D, C> {
    pub fn read(&mut self) -> Result<ApexConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x45, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<ApexConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x45, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(ApexConfig3Val(val))
    }
    pub fn write(&mut self, val: ApexConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x45, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(ApexConfig3Val(raw_val))
    }
    pub async fn write_async(&mut self, val: ApexConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x45, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig3Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(ApexConfig3Val) -> ApexConfig3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(ApexConfig3Val) -> ApexConfig3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(ApexConfig3Val(0x85))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(ApexConfig3Val(0x85)).await
    }
}
pub struct ApexConfig3Val(pub u8);
impl ApexConfig3Val {
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
        Self(0x85)
    }
    pub fn ped_amp_th_sel<'a>(&'a mut self) -> FieldPedAmpThSel<'a> {
        FieldPedAmpThSel(self)
    }
    pub fn ped_step_cnt_th_sel<'a>(&'a mut self) -> FieldPedStepCntThSel<'a> {
        FieldPedStepCntThSel(self)
    }
}
pub struct FieldPedAmpThSel<'a>(pub &'a mut ApexConfig3Val);
impl<'a> FieldPedAmpThSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig3Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig3Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= 0x85 & (!(!0 << 4) << 4);
        self.0
    }
}
pub struct FieldPedStepCntThSel<'a>(pub &'a mut ApexConfig3Val);
impl<'a> FieldPedStepCntThSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut ApexConfig3Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut ApexConfig3Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x85 & (!(!0 << 4) << 0);
        self.0
    }
}
