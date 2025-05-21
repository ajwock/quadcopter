use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct FifoLostPkt0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> FifoLostPkt0<'a, C> {
    pub fn read(&mut self) -> Result<FifoLostPkt0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x2f, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoLostPkt0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoLostPkt0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x2f, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoLostPkt0Val(val))
    }
}
pub struct FifoLostPkt0Val(pub u8);
impl FifoLostPkt0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn fifo_lost_pkt_cnt_7_0<'a>(&'a mut self) -> FifoLostPktCnt70<'a> {
        FifoLostPktCnt70(self)
    }
}
pub struct FifoLostPktCnt70<'a>(pub &'a mut FifoLostPkt0Val);
impl<'a> FifoLostPktCnt70<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
