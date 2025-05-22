use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct FsyncConfig<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> FsyncConfig<'a, C> {
    pub fn read(&mut self) -> Result<FsyncConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x3, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(FsyncConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FsyncConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x3, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FsyncConfigVal(val))
    }
    pub fn write(&mut self, val: FsyncConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x3, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FsyncConfigVal(raw_val))
    }
    pub async fn write_async(&mut self, val: FsyncConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x3, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FsyncConfigVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(FsyncConfigVal) -> FsyncConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FsyncConfigVal) -> FsyncConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FsyncConfigVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FsyncConfigVal(0x0)).await
    }
}
pub struct FsyncConfigVal(pub u8);
impl FsyncConfigVal {
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
    pub fn fsync_ui_sel<'a>(&'a mut self) -> FieldFsyncUiSel<'a> {
        FieldFsyncUiSel(self)
    }
    pub fn fsync_ui_flag_clear_sel<'a>(&'a mut self) -> FieldFsyncUiFlagClearSel<'a> {
        FieldFsyncUiFlagClearSel(self)
    }
    pub fn fsync_polarity<'a>(&'a mut self) -> FieldFsyncPolarity<'a> {
        FieldFsyncPolarity(self)
    }
}
pub struct FieldFsyncUiSel<'a>(pub &'a mut FsyncConfigVal);
impl<'a> FieldFsyncUiSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 4) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FsyncConfigVal {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 4;
        self.0
    }
    pub fn reset(self) -> &'a mut FsyncConfigVal {
        self.0.0 &= !(!(!0 << 3) << 4);
        self.0.0 |= 0x0 & (!(!0 << 3) << 4);
        self.0
    }
}
pub struct FieldFsyncUiFlagClearSel<'a>(pub &'a mut FsyncConfigVal);
impl<'a> FieldFsyncUiFlagClearSel<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsyncConfigVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsyncConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsyncConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsyncConfigVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldFsyncPolarity<'a>(pub &'a mut FsyncConfigVal);
impl<'a> FieldFsyncPolarity<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsyncConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsyncConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsyncConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsyncConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
