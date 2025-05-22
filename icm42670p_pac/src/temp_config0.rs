use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct TempConfig0<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> TempConfig0<'a, D, C> {
    pub fn read(&mut self) -> Result<TempConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x22, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(TempConfig0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<TempConfig0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x22, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(TempConfig0Val(val))
    }
    pub fn write(&mut self, val: TempConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x22, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(TempConfig0Val(raw_val))
    }
    pub async fn write_async(&mut self, val: TempConfig0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x22, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(TempConfig0Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(TempConfig0Val) -> TempConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(TempConfig0Val) -> TempConfig0Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(TempConfig0Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(TempConfig0Val(0x0)).await
    }
}
pub struct TempConfig0Val(pub u8);
impl TempConfig0Val {
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
    pub fn temp_filt_bw<'a>(&'a mut self) -> FieldTempFiltBw<'a> {
        FieldTempFiltBw(self)
    }
}
pub struct FieldTempFiltBw<'a>(pub &'a mut TempConfig0Val);
impl<'a> FieldTempFiltBw<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TempConfig0Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut TempConfig0Val {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= 0x0 & (!(!0 << 3) << 4);
        self.0
    }
}
