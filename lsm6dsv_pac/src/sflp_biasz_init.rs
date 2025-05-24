use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct SflpBiaszInit<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> SflpBiaszInit<'a, C> {
    pub fn read(&mut self) -> Result<SflpBiaszInitVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x36, &mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpBiaszInitVal(val))
    }
    pub async fn read_async(&mut self) -> Result<SflpBiaszInitVal, RegCommsError> {
        let mut buf = [0u8; 2];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x36, &mut buf).await?;
        let val = u16::from_le_bytes(buf);
        Ok(SflpBiaszInitVal(val))
    }
    pub fn write(&mut self, val: SflpBiaszInitVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x36, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u16) -> Result<(), RegCommsError> {
        self.write(SflpBiaszInitVal(raw_val))
    }
    pub async fn write_async(&mut self, val: SflpBiaszInitVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x36, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u16) -> Result<(), RegCommsError> {
        self.write_async(SflpBiaszInitVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(SflpBiaszInitVal) -> SflpBiaszInitVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(SflpBiaszInitVal) -> SflpBiaszInitVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(SflpBiaszInitVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(SflpBiaszInitVal(0x0)).await
    }
}
pub struct SflpBiaszInitVal(pub u16);
impl SflpBiaszInitVal {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn zero() -> Self {
        Self(0)
    }
    pub fn set(&mut self, val: u16) {
        self.0 = val;
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
}
