use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct TapDur<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> TapDur<'a, C> {
    pub fn read(&mut self) -> Result<TapDurVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x5a, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(TapDurVal(val))
    }
    pub async fn read_async(&mut self) -> Result<TapDurVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x5a, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(TapDurVal(val))
    }
    pub fn write(&mut self, val: TapDurVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x5a, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(TapDurVal(raw_val))
    }
    pub async fn write_async(&mut self, val: TapDurVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x5a, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(TapDurVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(TapDurVal) -> TapDurVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(TapDurVal) -> TapDurVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(TapDurVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(TapDurVal(0x0)).await
    }
}
pub struct TapDurVal(pub u8);
impl TapDurVal {
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
    pub fn dur<'a>(&'a mut self) -> FieldDur<'a> {
        FieldDur(self)
    }
    pub fn quiet<'a>(&'a mut self) -> FieldQuiet<'a> {
        FieldQuiet(self)
    }
    pub fn shock<'a>(&'a mut self) -> FieldShock<'a> {
        FieldShock(self)
    }
}
pub struct FieldDur<'a>(pub &'a mut TapDurVal);
impl<'a> FieldDur<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TapDurVal {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut TapDurVal {
        self.0.0 &= !(!(!0 << 4) << 4);
        self.0.0 |= 0x0 & (!(!0 << 4) << 4);
        self.0
    }
}
pub struct FieldQuiet<'a>(pub &'a mut TapDurVal);
impl<'a> FieldQuiet<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TapDurVal {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 2;
        self.0
    }
    pub fn reset(self) -> &'a mut TapDurVal {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= 0x0 & (!(!0 << 2) << 2);
        self.0
    }
}
pub struct FieldShock<'a>(pub &'a mut TapDurVal);
impl<'a> FieldShock<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut TapDurVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut TapDurVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x0 & (!(!0 << 2) << 0);
        self.0
    }
}
