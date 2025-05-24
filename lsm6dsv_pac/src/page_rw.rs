use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct PageRw<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> PageRw<'a, C> {
    pub fn read(&mut self) -> Result<PageRwVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x17, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(PageRwVal(val))
    }
    pub async fn read_async(&mut self) -> Result<PageRwVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x17, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(PageRwVal(val))
    }
    pub fn write(&mut self, val: PageRwVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x17, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(PageRwVal(raw_val))
    }
    pub async fn write_async(&mut self, val: PageRwVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x17, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(PageRwVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(PageRwVal) -> PageRwVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(PageRwVal) -> PageRwVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(PageRwVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(PageRwVal(0x0)).await
    }
}
pub struct PageRwVal(pub u8);
impl PageRwVal {
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
    pub fn emb_func_lir<'a>(&'a mut self) -> FieldEmbFuncLir<'a> {
        FieldEmbFuncLir(self)
    }
    pub fn page_write<'a>(&'a mut self) -> FieldPageWrite<'a> {
        FieldPageWrite(self)
    }
    pub fn page_read<'a>(&'a mut self) -> FieldPageRead<'a> {
        FieldPageRead(self)
    }
}
pub struct FieldEmbFuncLir<'a>(pub &'a mut PageRwVal);
impl<'a> FieldEmbFuncLir<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PageRwVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut PageRwVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PageRwVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut PageRwVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldPageWrite<'a>(pub &'a mut PageRwVal);
impl<'a> FieldPageWrite<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PageRwVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut PageRwVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PageRwVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut PageRwVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldPageRead<'a>(pub &'a mut PageRwVal);
impl<'a> FieldPageRead<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PageRwVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut PageRwVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PageRwVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut PageRwVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
