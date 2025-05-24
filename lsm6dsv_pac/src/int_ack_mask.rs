use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct IntAckMask<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> IntAckMask<'a, C> {
    pub fn read(&mut self) -> Result<IntAckMaskVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x4b, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(IntAckMaskVal(val))
    }
    pub async fn read_async(&mut self) -> Result<IntAckMaskVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x4b, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(IntAckMaskVal(val))
    }
    pub fn write(&mut self, val: IntAckMaskVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0x4b, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntAckMaskVal(raw_val))
    }
    pub async fn write_async(&mut self, val: IntAckMaskVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0x4b, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntAckMaskVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntAckMaskVal) -> IntAckMaskVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntAckMaskVal) -> IntAckMaskVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntAckMaskVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntAckMaskVal(0x0)).await
    }
}
pub struct IntAckMaskVal(pub u8);
impl IntAckMaskVal {
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
    pub fn iack_mask8<'a>(&'a mut self) -> FieldIackMask8<'a> {
        FieldIackMask8(self)
    }
    pub fn iack_mask7<'a>(&'a mut self) -> FieldIackMask7<'a> {
        FieldIackMask7(self)
    }
    pub fn iack_mask6<'a>(&'a mut self) -> FieldIackMask6<'a> {
        FieldIackMask6(self)
    }
    pub fn iack_mask5<'a>(&'a mut self) -> FieldIackMask5<'a> {
        FieldIackMask5(self)
    }
    pub fn iack_mask4<'a>(&'a mut self) -> FieldIackMask4<'a> {
        FieldIackMask4(self)
    }
    pub fn iack_mask3<'a>(&'a mut self) -> FieldIackMask3<'a> {
        FieldIackMask3(self)
    }
    pub fn iack_mask2<'a>(&'a mut self) -> FieldIackMask2<'a> {
        FieldIackMask2(self)
    }
    pub fn iack_mask1<'a>(&'a mut self) -> FieldIackMask1<'a> {
        FieldIackMask1(self)
    }
}
pub struct FieldIackMask8<'a>(pub &'a mut IntAckMaskVal);
impl<'a> FieldIackMask8<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldIackMask7<'a>(pub &'a mut IntAckMaskVal);
impl<'a> FieldIackMask7<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldIackMask6<'a>(pub &'a mut IntAckMaskVal);
impl<'a> FieldIackMask6<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldIackMask5<'a>(pub &'a mut IntAckMaskVal);
impl<'a> FieldIackMask5<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldIackMask4<'a>(pub &'a mut IntAckMaskVal);
impl<'a> FieldIackMask4<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldIackMask3<'a>(pub &'a mut IntAckMaskVal);
impl<'a> FieldIackMask3<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldIackMask2<'a>(pub &'a mut IntAckMaskVal);
impl<'a> FieldIackMask2<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldIackMask1<'a>(pub &'a mut IntAckMaskVal);
impl<'a> FieldIackMask1<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntAckMaskVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntAckMaskVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
