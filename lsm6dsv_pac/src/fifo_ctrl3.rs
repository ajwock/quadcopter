use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FifoCtrl3<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FifoCtrl3<'a, C> {
    pub fn read(&mut self) -> Result<FifoCtrl3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x9, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoCtrl3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoCtrl3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x9, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoCtrl3Val(val))
    }
    pub fn write(&mut self, val: FifoCtrl3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x9, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FifoCtrl3Val(raw_val))
    }
    pub async fn write_async(&mut self, val: FifoCtrl3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x9, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FifoCtrl3Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(FifoCtrl3Val) -> FifoCtrl3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FifoCtrl3Val) -> FifoCtrl3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FifoCtrl3Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FifoCtrl3Val(0x0)).await
    }
}
pub struct FifoCtrl3Val(pub u8);
impl FifoCtrl3Val {
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
    pub fn bdr_gy<'a>(&'a mut self) -> FieldBdrGy<'a> {
        FieldBdrGy(self)
    }
    pub fn bdr_xl<'a>(&'a mut self) -> FieldBdrXl<'a> {
        FieldBdrXl(self)
    }
}
pub struct FieldBdrGy<'a>(pub &'a mut FifoCtrl3Val);
impl<'a> FieldBdrGy<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoCtrl3Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut FifoCtrl3Val {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= 0x0 & (!(!0 << 4) << 4);
        self.0
    }
}
pub struct FieldBdrXl<'a>(pub &'a mut FifoCtrl3Val);
impl<'a> FieldBdrXl<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoCtrl3Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut FifoCtrl3Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= 0x0 & (!(!0 << 4) << 0);
        self.0
    }
}
