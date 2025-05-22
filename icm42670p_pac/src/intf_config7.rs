use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntfConfig7<'a, C: RegComms<1, u8>>(pub &'a mut Icm42670P<C>);
impl<'a, C: RegComms<1, u8>> IntfConfig7<'a, C> {
    pub fn read(&mut self) -> Result<IntfConfig7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read(&mut self.0, 0x28, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig7Val(val))
    }
    pub async fn read_async(&mut self) -> Result<IntfConfig7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.mreg_1;
        proc.proc_read_async(&mut self.0, 0x28, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntfConfig7Val(val))
    }
    pub fn write(&mut self, val: IntfConfig7Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write(&mut self.0, 0x28, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntfConfig7Val(raw_val))
    }
    pub async fn write_async(&mut self, val: IntfConfig7Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.mreg_1;
        proc.proc_write_async(&mut self.0, 0x28, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig7Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntfConfig7Val) -> IntfConfig7Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntfConfig7Val) -> IntfConfig7Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntfConfig7Val(0x28))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntfConfig7Val(0x28)).await
    }
}
pub struct IntfConfig7Val(pub u8);
impl IntfConfig7Val {
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
        Self(0x28)
    }
    pub fn i3_c_ddr_wr_mode<'a>(&'a mut self) -> FieldI3CDdrWrMode<'a> {
        FieldI3CDdrWrMode(self)
    }
}
pub struct FieldI3CDdrWrMode<'a>(pub &'a mut IntfConfig7Val);
impl<'a> FieldI3CDdrWrMode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntfConfig7Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntfConfig7Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntfConfig7Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntfConfig7Val {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x28;
        self.0
    }
}
