use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct TapCfg2<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> TapCfg2<'a, C> {
    pub fn read(&mut self) -> Result<TapCfg2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x58, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(TapCfg2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<TapCfg2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x58, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(TapCfg2Val(val))
    }
    pub fn write(&mut self, val: TapCfg2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x58, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(TapCfg2Val(raw_val))
    }
    pub async fn write_async(&mut self, val: TapCfg2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x58, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(TapCfg2Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(TapCfg2Val) -> TapCfg2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(TapCfg2Val) -> TapCfg2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(TapCfg2Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(TapCfg2Val(0x0)).await
    }
}
pub struct TapCfg2Val(pub u8);
impl TapCfg2Val {
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
    pub fn tap_ths_y<'a>(&'a mut self) -> FieldTapThsY<'a> {
        FieldTapThsY(self)
    }
}
pub struct FieldTapThsY<'a>(pub &'a mut TapCfg2Val);
impl<'a> FieldTapThsY<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 5)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TapCfg2Val {
        self.0.0 &= !(!(!0 << 5) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 5)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut TapCfg2Val {
        self.0.0 &= !(!(!0 << 5) << 0);
        self.0.0 |= 0x0 & (!(!0 << 5) << 0);
        self.0
    }
}
