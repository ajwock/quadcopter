use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct FifoConfig6<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> FifoConfig6<'a, D, C> {
    pub fn read(&mut self) -> Result<FifoConfig6Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x2, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig6Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoConfig6Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x2, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig6Val(val))
    }
    pub fn write(&mut self, val: FifoConfig6Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x2, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(FifoConfig6Val(raw_val))
    }
    pub async fn write_async(&mut self, val: FifoConfig6Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x2, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(FifoConfig6Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(FifoConfig6Val) -> FifoConfig6Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(FifoConfig6Val) -> FifoConfig6Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(FifoConfig6Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(FifoConfig6Val(0x0)).await
    }
}
pub struct FifoConfig6Val(pub u8);
impl FifoConfig6Val {
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
    pub fn fifo_empty_indicator_dis<'a>(&'a mut self) -> FieldFifoEmptyIndicatorDis<'a> {
        FieldFifoEmptyIndicatorDis(self)
    }
    pub fn rcosc_req_on_fifo_ths_dis<'a>(&'a mut self) -> FieldRcoscReqOnFifoThsDis<'a> {
        FieldRcoscReqOnFifoThsDis(self)
    }
}
pub struct FieldFifoEmptyIndicatorDis<'a>(pub &'a mut FifoConfig6Val);
impl<'a> FieldFifoEmptyIndicatorDis<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig6Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig6Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig6Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoConfig6Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldRcoscReqOnFifoThsDis<'a>(pub &'a mut FifoConfig6Val);
impl<'a> FieldRcoscReqOnFifoThsDis<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfig6Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfig6Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfig6Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut FifoConfig6Val {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
