use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FsmInt2<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FsmInt2<'a, C> {
    pub fn read(&mut self) -> Result<FsmInt2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0xf, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FsmInt2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FsmInt2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0xf, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FsmInt2Val(val))
    }
    pub fn write(&mut self, val: FsmInt2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write(&mut self.0, 0xf, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FsmInt2Val(raw_val))
    }
    pub async fn write_async(&mut self, val: FsmInt2Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.embedded_func;
        proc.proc_write_async(&mut self.0, 0xf, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FsmInt2Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(FsmInt2Val) -> FsmInt2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FsmInt2Val) -> FsmInt2Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FsmInt2Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FsmInt2Val(0x0)).await
    }
}
pub struct FsmInt2Val(pub u8);
impl FsmInt2Val {
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
    pub fn int2_fsm8<'a>(&'a mut self) -> FieldInt2Fsm8<'a> {
        FieldInt2Fsm8(self)
    }
    pub fn int2_fsm7<'a>(&'a mut self) -> FieldInt2Fsm7<'a> {
        FieldInt2Fsm7(self)
    }
    pub fn int2_fsm6<'a>(&'a mut self) -> FieldInt2Fsm6<'a> {
        FieldInt2Fsm6(self)
    }
    pub fn int2_fsm5<'a>(&'a mut self) -> FieldInt2Fsm5<'a> {
        FieldInt2Fsm5(self)
    }
    pub fn int2_fsm4<'a>(&'a mut self) -> FieldInt2Fsm4<'a> {
        FieldInt2Fsm4(self)
    }
    pub fn int2_fsm3<'a>(&'a mut self) -> FieldInt2Fsm3<'a> {
        FieldInt2Fsm3(self)
    }
    pub fn int2_fsm2<'a>(&'a mut self) -> FieldInt2Fsm2<'a> {
        FieldInt2Fsm2(self)
    }
    pub fn int2_fsm1<'a>(&'a mut self) -> FieldInt2Fsm1<'a> {
        FieldInt2Fsm1(self)
    }
}
pub struct FieldInt2Fsm8<'a>(pub &'a mut FsmInt2Val);
impl<'a> FieldInt2Fsm8<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (val as u8) << 7;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= (1 << 7) & 0x0;
        self.0
    }
}
pub struct FieldInt2Fsm7<'a>(pub &'a mut FsmInt2Val);
impl<'a> FieldInt2Fsm7<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (val as u8) << 6;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= (1 << 6) & 0x0;
        self.0
    }
}
pub struct FieldInt2Fsm6<'a>(pub &'a mut FsmInt2Val);
impl<'a> FieldInt2Fsm6<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt2Fsm5<'a>(pub &'a mut FsmInt2Val);
impl<'a> FieldInt2Fsm5<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldInt2Fsm4<'a>(pub &'a mut FsmInt2Val);
impl<'a> FieldInt2Fsm4<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldInt2Fsm3<'a>(pub &'a mut FsmInt2Val);
impl<'a> FieldInt2Fsm3<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldInt2Fsm2<'a>(pub &'a mut FsmInt2Val);
impl<'a> FieldInt2Fsm2<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldInt2Fsm1<'a>(pub &'a mut FsmInt2Val);
impl<'a> FieldInt2Fsm1<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FsmInt2Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FsmInt2Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FsmInt2Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
