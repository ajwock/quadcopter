use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct FifoLostPkt0<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> FifoLostPkt0<'a, D, C> {
    pub fn read(&mut self) -> Result<FifoLostPkt0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x2f, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoLostPkt0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoLostPkt0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x2f, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoLostPkt0Val(val))
    }
}
pub struct FifoLostPkt0Val(pub u8);
impl FifoLostPkt0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn reset_val() -> Self {
        Self(0x0)
    }
    pub fn fifo_lost_pkt_cnt_7_0<'a>(&'a mut self) -> FieldFifoLostPktCnt70<'a> {
        FieldFifoLostPktCnt70(self)
    }
}
pub struct FieldFifoLostPktCnt70<'a>(pub &'a mut FifoLostPkt0Val);
impl<'a> FieldFifoLostPktCnt70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
