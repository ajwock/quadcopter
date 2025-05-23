use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FifoStatus2<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FifoStatus2<'a, C> {
    pub fn read(&mut self) -> Result<FifoStatus2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x1c, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoStatus2Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoStatus2Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x1c, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FifoStatus2Val(val))
    }
}
pub struct FifoStatus2Val(pub u8);
impl FifoStatus2Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn fifo_wtm_ia<'a>(&'a mut self) -> FieldFifoWtmIa<'a> {
        FieldFifoWtmIa(self)
    }
    pub fn fifo_ovr_ia<'a>(&'a mut self) -> FieldFifoOvrIa<'a> {
        FieldFifoOvrIa(self)
    }
    pub fn fifo_full_a<'a>(&'a mut self) -> FieldFifoFullA<'a> {
        FieldFifoFullA(self)
    }
    pub fn counter_bdr_ia<'a>(&'a mut self) -> FieldCounterBdrIa<'a> {
        FieldCounterBdrIa(self)
    }
    pub fn fifo_ovr_latched<'a>(&'a mut self) -> FieldFifoOvrLatched<'a> {
        FieldFifoOvrLatched(self)
    }
    pub fn diff_fifo_8<'a>(&'a mut self) -> FieldDiffFifo8<'a> {
        FieldDiffFifo8(self)
    }
}
pub struct FieldFifoWtmIa<'a>(pub &'a mut FifoStatus2Val);
impl<'a> FieldFifoWtmIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFifoOvrIa<'a>(pub &'a mut FifoStatus2Val);
impl<'a> FieldFifoOvrIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFifoFullA<'a>(pub &'a mut FifoStatus2Val);
impl<'a> FieldFifoFullA<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldCounterBdrIa<'a>(pub &'a mut FifoStatus2Val);
impl<'a> FieldCounterBdrIa<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFifoOvrLatched<'a>(pub &'a mut FifoStatus2Val);
impl<'a> FieldFifoOvrLatched<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldDiffFifo8<'a>(pub &'a mut FifoStatus2Val);
impl<'a> FieldDiffFifo8<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
