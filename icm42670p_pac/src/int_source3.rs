use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct IntSource3<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntSource3<'a, C> {
    pub fn read(&mut self) -> Result<IntSource3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x2d, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntSource3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x2d, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntSource3Val(val))
    }
    pub fn write(&mut self, val: IntSource3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x2d, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: IntSource3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x2d, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct IntSource3Val(pub u8);
impl IntSource3Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn st_int2_en<'a>(&'a mut self) -> StInt2En<'a> {
        StInt2En(self)
    }
    pub fn fsync_int2_en<'a>(&'a mut self) -> FsyncInt2En<'a> {
        FsyncInt2En(self)
    }
    pub fn pll_rdy_int2_en<'a>(&'a mut self) -> PllRdyInt2En<'a> {
        PllRdyInt2En(self)
    }
    pub fn reset_done_int2_en<'a>(&'a mut self) -> ResetDoneInt2En<'a> {
        ResetDoneInt2En(self)
    }
    pub fn drdy_int2_en<'a>(&'a mut self) -> DrdyInt2En<'a> {
        DrdyInt2En(self)
    }
    pub fn fifo_ths_int2_en<'a>(&'a mut self) -> FifoThsInt2En<'a> {
        FifoThsInt2En(self)
    }
    pub fn fifo_full_int2_en<'a>(&'a mut self) -> FifoFullInt2En<'a> {
        FifoFullInt2En(self)
    }
    pub fn agc_rdy_int2_en<'a>(&'a mut self) -> AgcRdyInt2En<'a> {
        AgcRdyInt2En(self)
    }
}
pub struct StInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> StInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= !(!(val as u8) << 7);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
}
pub struct FsyncInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FsyncInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 6);
        self.0.0 |= !(!(val as u8) << 6);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
}
pub struct PllRdyInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> PllRdyInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 5);
        self.0.0 |= !(!(val as u8) << 5);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
}
pub struct ResetDoneInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> ResetDoneInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= !(!(val as u8) << 4);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
}
pub struct DrdyInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> DrdyInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= !(!(val as u8) << 3);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
}
pub struct FifoThsInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FifoThsInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
}
pub struct FifoFullInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> FifoFullInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 1);
        self.0.0 |= !(!(val as u8) << 1);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
}
pub struct AgcRdyInt2En<'a>(pub &'a mut IntSource3Val);
impl<'a> AgcRdyInt2En<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntSource3Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= !(!(val as u8) << 0);
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntSource3Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntSource3Val {
        self.assign(false)
    }
}
