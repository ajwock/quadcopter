use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntStatus<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntStatus<'a, C> {
    pub fn read(&mut self) -> Result<IntStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x3a, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatusVal(val))
    }
    pub async fn read_async(&mut self) -> Result<IntStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x3a, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatusVal(val))
    }
}
pub struct IntStatusVal(pub u8);
impl IntStatusVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x10)
    }
    pub fn st_int<'a>(&'a mut self) -> FieldStInt<'a> {
        FieldStInt(self)
    }
    pub fn fsync_int<'a>(&'a mut self) -> FieldFsyncInt<'a> {
        FieldFsyncInt(self)
    }
    pub fn pll_rdy_int<'a>(&'a mut self) -> FieldPllRdyInt<'a> {
        FieldPllRdyInt(self)
    }
    pub fn reset_done_int<'a>(&'a mut self) -> FieldResetDoneInt<'a> {
        FieldResetDoneInt(self)
    }
    pub fn fifo_ths_int<'a>(&'a mut self) -> FieldFifoThsInt<'a> {
        FieldFifoThsInt(self)
    }
    pub fn fifo_full_int<'a>(&'a mut self) -> FieldFifoFullInt<'a> {
        FieldFifoFullInt(self)
    }
    pub fn agc_rdy_int<'a>(&'a mut self) -> FieldAgcRdyInt<'a> {
        FieldAgcRdyInt(self)
    }
}
pub struct FieldStInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FieldStInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFsyncInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FieldFsyncInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldPllRdyInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FieldPllRdyInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldResetDoneInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FieldResetDoneInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFifoThsInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FieldFifoThsInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldFifoFullInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FieldFifoFullInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldAgcRdyInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FieldAgcRdyInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
