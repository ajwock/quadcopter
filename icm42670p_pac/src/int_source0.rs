use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntSource0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource0<'a, C> {
    pub fn read(&mut self) -> Result<IntSource0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x2b, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x2b, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource0Val(val))
    }
    pub fn write(&mut self, val: IntSource0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x2b, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: IntSource0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x2b, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct IntSource0Val(pub u8);
impl IntSource0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn st_int1_en<'a>(&'a mut self) -> StInt1En<'a> {
        StInt1En(self)
    }
    pub fn fsync_int1_en<'a>(&'a mut self) -> FsyncInt1En<'a> {
        FsyncInt1En(self)
    }
    pub fn pll_rdy_int1_en<'a>(&'a mut self) -> PllRdyInt1En<'a> {
        PllRdyInt1En(self)
    }
    pub fn reset_done_int1_en<'a>(&'a mut self) -> ResetDoneInt1En<'a> {
        ResetDoneInt1En(self)
    }
    pub fn drdy_int1_en<'a>(&'a mut self) -> DrdyInt1En<'a> {
        DrdyInt1En(self)
    }
    pub fn fifo_ths_int1_en<'a>(&'a mut self) -> FifoThsInt1En<'a> {
        FifoThsInt1En(self)
    }
    pub fn fifo_full_int1_en<'a>(&'a mut self) -> FifoFullInt1En<'a> {
        FifoFullInt1En(self)
    }
    pub fn agc_rdy_int1_en<'a>(&'a mut self) -> AgcRdyInt1En<'a> {
        AgcRdyInt1En(self)
    }
}
pub struct StInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> StInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= !(!(val as u8) << 7);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
}
pub struct FsyncInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FsyncInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= !(!(val as u8) << 6);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
}
pub struct PllRdyInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> PllRdyInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= !(!(val as u8) << 5);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
}
pub struct ResetDoneInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> ResetDoneInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= !(!(val as u8) << 4);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
}
pub struct DrdyInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> DrdyInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= !(!(val as u8) << 3);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
}
pub struct FifoThsInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FifoThsInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
}
pub struct FifoFullInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> FifoFullInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= !(!(val as u8) << 1);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
}
pub struct AgcRdyInt1En<'a>(pub &'a mut IntSource0Val);
impl<'a> AgcRdyInt1En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource0Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= !(!(val as u8) << 0);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource0Val {
        self.assign(false)
    }
}
