use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct FifoLostPkt1<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> FifoLostPkt1<'a, C> {
    pub fn read(&mut self) -> Result<FifoLostPkt1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x30, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoLostPkt1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoLostPkt1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x30, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoLostPkt1Val(val))
    }
}
pub struct FifoLostPkt1Val(pub u8);
impl FifoLostPkt1Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn fifo_lost_pkt_cnt_15_8<'a>(&'a mut self) -> FifoLostPktCnt158<'a> {
        FifoLostPktCnt158(self)
    }
}
pub struct FifoLostPktCnt158<'a>(pub &'a mut FifoLostPkt1Val);
impl<'a> FifoLostPktCnt158<'a> {
    pub fn bits(&self) -> u8 {
        self.0.0
    }
}
