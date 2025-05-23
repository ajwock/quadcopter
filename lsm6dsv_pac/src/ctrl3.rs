use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct Ctrl3<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> Ctrl3<'a, C> {
    pub fn read(&mut self) -> Result<Ctrl3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x12, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<Ctrl3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x12, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(Ctrl3Val(val))
    }
    pub fn write(&mut self, val: Ctrl3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x12, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(Ctrl3Val(raw_val))
    }
    pub async fn write_async(&mut self, val: Ctrl3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x12, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(Ctrl3Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(Ctrl3Val) -> Ctrl3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(Ctrl3Val) -> Ctrl3Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(Ctrl3Val(0x44))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(Ctrl3Val(0x44)).await
    }
}
pub struct Ctrl3Val(pub u8);
impl Ctrl3Val {
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
        Self(0x44)
    }
    pub fn boot<'a>(&'a mut self) -> FieldBoot<'a> {
        FieldBoot(self)
    }
    pub fn bdu<'a>(&'a mut self) -> FieldBdu<'a> {
        FieldBdu(self)
    }
    pub fn if_inc<'a>(&'a mut self) -> FieldIfInc<'a> {
        FieldIfInc(self)
    }
    pub fn sw_reset<'a>(&'a mut self) -> FieldSwReset<'a> {
        FieldSwReset(self)
    }
}
pub struct FieldBoot<'a>(pub &'a mut Ctrl3Val);
impl<'a> FieldBoot<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl3Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl3Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x44;
        self.0
    }
}
pub struct FieldBdu<'a>(pub &'a mut Ctrl3Val);
impl<'a> FieldBdu<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl3Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl3Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x44;
        self.0
    }
}
pub struct FieldIfInc<'a>(pub &'a mut Ctrl3Val);
impl<'a> FieldIfInc<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl3Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl3Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x44;
        self.0
    }
}
pub struct FieldSwReset<'a>(pub &'a mut Ctrl3Val);
impl<'a> FieldSwReset<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut Ctrl3Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut Ctrl3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut Ctrl3Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut Ctrl3Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x44;
        self.0
    }
}
