use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl5<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl5<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x14, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl5Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl5Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x14, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl5Val(val))
    }
    pub fn write(&mut self, val: Ctrl5Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x14, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl5Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl5Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x14, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl5Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl5Val) -> Ctrl5Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl5Val) -> Ctrl5Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl5Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl5Val(0x0)).await
    }
}
pub struct Ctrl5Val(pub u8);
impl Ctrl5Val {
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
    pub fn bus_act_sel<'a>(&'a mut self) -> FieldBusActSel<'a> {
        FieldBusActSel(self)
    }
    pub fn int_en_i3_c<'a>(&'a mut self) -> FieldIntEnI3C<'a> {
        FieldIntEnI3C(self)
    }
}
pub struct FieldBusActSel<'a>(pub &'a mut Ctrl5Val);
impl<'a> FieldBusActSel<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 1) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut Ctrl5Val {
        self.0.0 &= !(!(!0 << 2) << 1);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 1;
        self.0
    }
    pub fn reset(self) -> &'a mut Ctrl5Val {
        self.0.0 &= !(!(!0 << 2) << 1);
        self.0.0 |= 0x0 & (!(!0 << 2) << 1);
        self.0
    }
}
pub struct FieldIntEnI3C<'a>(pub &'a mut Ctrl5Val);
impl<'a> FieldIntEnI3C<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl5Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl5Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl5Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl5Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
