use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntStatus<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntStatus<'a, C> {
    pub fn read(&mut self) -> Result<IntStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x3a, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatusVal(val))
    }
    pub async fn read_async(&mut self) -> Result<IntStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x3a, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntStatusVal(val))
    }
}
pub struct IntStatusVal(pub u8);
impl IntStatusVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn st_int<'a>(&'a mut self) -> StInt<'a> {
        StInt(self)
    }
    pub fn fsync_int<'a>(&'a mut self) -> FsyncInt<'a> {
        FsyncInt(self)
    }
    pub fn pll_rdy_int<'a>(&'a mut self) -> PllRdyInt<'a> {
        PllRdyInt(self)
    }
    pub fn reset_done_int<'a>(&'a mut self) -> ResetDoneInt<'a> {
        ResetDoneInt(self)
    }
    pub fn fifo_ths_int<'a>(&'a mut self) -> FifoThsInt<'a> {
        FifoThsInt(self)
    }
    pub fn fifo_full_int<'a>(&'a mut self) -> FifoFullInt<'a> {
        FifoFullInt(self)
    }
    pub fn agc_rdy_int<'a>(&'a mut self) -> AgcRdyInt<'a> {
        AgcRdyInt(self)
    }
}
pub struct StInt<'a>(pub &'a mut IntStatusVal);
impl<'a> StInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FsyncInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FsyncInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct PllRdyInt<'a>(pub &'a mut IntStatusVal);
impl<'a> PllRdyInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct ResetDoneInt<'a>(pub &'a mut IntStatusVal);
impl<'a> ResetDoneInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FifoThsInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FifoThsInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FifoFullInt<'a>(pub &'a mut IntStatusVal);
impl<'a> FifoFullInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct AgcRdyInt<'a>(pub &'a mut IntStatusVal);
impl<'a> AgcRdyInt<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
