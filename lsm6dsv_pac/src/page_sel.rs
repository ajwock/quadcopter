use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct PageSel<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> PageSel<'a, C> {
    pub fn read(&mut self) -> Result<PageSelVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x2, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(PageSelVal(val))
    }
    pub async fn read_async(&mut self) -> Result<PageSelVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x2, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(PageSelVal(val))
    }
    pub fn write(&mut self, val: PageSelVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x2, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(PageSelVal(raw_val))
    }
    pub async fn write_async(&mut self, val: PageSelVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x2, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(PageSelVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(PageSelVal) -> PageSelVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(PageSelVal) -> PageSelVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(PageSelVal(0x1))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(PageSelVal(0x1)).await
    }
}
pub struct PageSelVal(pub u8);
impl PageSelVal {
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
        Self(0x1)
    }
    pub fn page_sel<'a>(&'a mut self) -> FieldPageSel<'a> {
        FieldPageSel(self)
    }
}
pub struct FieldPageSel<'a>(pub &'a mut PageSelVal);
impl<'a> FieldPageSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut PageSelVal {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut PageSelVal {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= 0x1 & (!(!0 << 4) << 4);
        self.0
    }
}
