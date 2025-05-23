use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct TapCfg1<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> TapCfg1<'a, C> {
    pub fn read(&mut self) -> Result<TapCfg1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x57, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(TapCfg1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<TapCfg1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x57, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(TapCfg1Val(val))
    }
    pub fn write(&mut self, val: TapCfg1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x57, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(TapCfg1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: TapCfg1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x57, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(TapCfg1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(TapCfg1Val) -> TapCfg1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(TapCfg1Val) -> TapCfg1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(TapCfg1Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(TapCfg1Val(0x0)).await
    }
}
pub struct TapCfg1Val(pub u8);
impl TapCfg1Val {
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
    pub fn tap_priority<'a>(&'a mut self) -> FieldTapPriority<'a> {
        FieldTapPriority(self)
    }
    pub fn tap_ths_x<'a>(&'a mut self) -> FieldTapThsX<'a> {
        FieldTapThsX(self)
    }
}
pub struct FieldTapPriority<'a>(pub &'a mut TapCfg1Val);
impl<'a> FieldTapPriority<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TapCfg1Val {
        self.0.0 &= !(!(!0 << 3) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 5;
        self.0
    }
    pub fn reset(self) -> &'a mut TapCfg1Val {
        self.0.0 &= !(!(!0 << 3) << 5);
        self.0.0 |= 0x0 & (!(!0 << 3) << 5);
        self.0
    }
}
pub struct FieldTapThsX<'a>(pub &'a mut TapCfg1Val);
impl<'a> FieldTapThsX<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 5)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TapCfg1Val {
        self.0.0 &= !(!(!0 << 5) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 5)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut TapCfg1Val {
        self.0.0 &= !(!(!0 << 5) << 0);
        self.0.0 |= 0x0 & (!(!0 << 5) << 0);
        self.0
    }
}
