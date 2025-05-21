use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct FifoConfig3<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> FifoConfig3<'a, C> {
    pub fn read(&mut self) -> Result<FifoConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x2a, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig3Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoConfig3Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x2a, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfig3Val(val))
    }
    pub fn write(&mut self, val: FifoConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x2a, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: FifoConfig3Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x2a, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct FifoConfig3Val(pub u8);
impl FifoConfig3Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn fifo_wm_11_8<'a>(&'a mut self) -> FifoWm118<'a> {
        FifoWm118(self)
    }
}
pub struct FifoWm118<'a>(pub &'a mut FifoConfig3Val);
impl<'a> FifoWm118<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 4)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoConfig3Val {
        self.0.0 &= !(!(!0 << 4) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 4)) << 0;
        self.0
    }
}
