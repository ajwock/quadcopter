use core::result::Result;
use regcomms::{RegCommsError, RegComms};
use crate::Icm42670P;
pub struct PwrMgmt0<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> PwrMgmt0<'a, C> {
    pub fn read(&mut self) -> Result<PwrMgmt0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x1f, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(PwrMgmt0Val(val))
    }
    pub async fn read_async(&mut self) -> Result<PwrMgmt0Val, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x1f, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(PwrMgmt0Val(val))
    }
    pub fn write(&mut self, val: PwrMgmt0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x1f, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: PwrMgmt0Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x1f, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct PwrMgmt0Val(pub u8);
impl PwrMgmt0Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn accel_lp_clk_sel<'a>(&'a mut self) -> AccelLpClkSel<'a> {
        AccelLpClkSel(self)
    }
    pub fn idle<'a>(&'a mut self) -> Idle<'a> {
        Idle(self)
    }
    pub fn gyro_mode<'a>(&'a mut self) -> GyroMode<'a> {
        GyroMode(self)
    }
    pub fn accel_mode<'a>(&'a mut self) -> AccelMode<'a> {
        AccelMode(self)
    }
}
pub struct AccelLpClkSel<'a>(pub &'a mut PwrMgmt0Val);
impl<'a> AccelLpClkSel<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(1 << 7);
        self.0.0 |= !(!(val as u8) << 7);
        self.0
    }
    pub fn set_bit(self) -> &'a mut PwrMgmt0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PwrMgmt0Val {
        self.assign(false)
    }
}
pub struct Idle<'a>(pub &'a mut PwrMgmt0Val);
impl<'a> Idle<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= !(!(val as u8) << 4);
        self.0
    }
    pub fn set_bit(self) -> &'a mut PwrMgmt0Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PwrMgmt0Val {
        self.assign(false)
    }
}
pub struct GyroMode<'a>(pub &'a mut PwrMgmt0Val);
impl<'a> GyroMode<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(!(!0 << 2) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 2;
        self.0
    }
}
pub struct AccelMode<'a>(pub &'a mut PwrMgmt0Val);
impl<'a> AccelMode<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut PwrMgmt0Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
}
