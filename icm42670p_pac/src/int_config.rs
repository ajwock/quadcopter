use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Icm42670P;
pub struct IntConfig<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>>(pub &'a mut Icm42670P<D, C>);
impl<'a, D: embedded_hal_async::delay::DelayNs, C: RegComms<1, u8>> IntConfig<'a, D, C> {
    pub fn read(&mut self) -> Result<IntConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0x6, &mut buf)?;
        let val = u8::from_be_bytes(buf);
        Ok(IntConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<IntConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0x6, &mut buf).await?;
        let val = u8::from_be_bytes(buf);
        Ok(IntConfigVal(val))
    }
    pub fn write(&mut self, val: IntConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0x6, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(IntConfigVal(raw_val))
    }
    pub async fn write_async(&mut self, val: IntConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0x6, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(IntConfigVal(raw_val)).await
    }
    pub fn modify<F: FnOnce(IntConfigVal) -> IntConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(IntConfigVal) -> IntConfigVal>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(IntConfigVal(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(IntConfigVal(0x0)).await
    }
}
pub struct IntConfigVal(pub u8);
impl IntConfigVal {
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
    pub fn int2_mode<'a>(&'a mut self) -> FieldInt2Mode<'a> {
        FieldInt2Mode(self)
    }
    pub fn int2_drive_circuit<'a>(&'a mut self) -> FieldInt2DriveCircuit<'a> {
        FieldInt2DriveCircuit(self)
    }
    pub fn int2_polarity<'a>(&'a mut self) -> FieldInt2Polarity<'a> {
        FieldInt2Polarity(self)
    }
    pub fn int1_mode<'a>(&'a mut self) -> FieldInt1Mode<'a> {
        FieldInt1Mode(self)
    }
    pub fn int1_drive_circuit<'a>(&'a mut self) -> FieldInt1DriveCircuit<'a> {
        FieldInt1DriveCircuit(self)
    }
    pub fn int1_polarity<'a>(&'a mut self) -> FieldInt1Polarity<'a> {
        FieldInt1Polarity(self)
    }
}
pub struct FieldInt2Mode<'a>(pub &'a mut IntConfigVal);
impl<'a> FieldInt2Mode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (val as u8) << 5;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 5);
        self.0.0 |= (1 << 5) & 0x0;
        self.0
    }
}
pub struct FieldInt2DriveCircuit<'a>(pub &'a mut IntConfigVal);
impl<'a> FieldInt2DriveCircuit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldInt2Polarity<'a>(pub &'a mut IntConfigVal);
impl<'a> FieldInt2Polarity<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (val as u8) << 3;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 3);
        self.0.0 |= (1 << 3) & 0x0;
        self.0
    }
}
pub struct FieldInt1Mode<'a>(pub &'a mut IntConfigVal);
impl<'a> FieldInt1Mode<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (val as u8) << 2;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= (1 << 2) & 0x0;
        self.0
    }
}
pub struct FieldInt1DriveCircuit<'a>(pub &'a mut IntConfigVal);
impl<'a> FieldInt1DriveCircuit<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (val as u8) << 1;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 1);
        self.0.0 |= (1 << 1) & 0x0;
        self.0
    }
}
pub struct FieldInt1Polarity<'a>(pub &'a mut IntConfigVal);
impl<'a> FieldInt1Polarity<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (val as u8) << 0;
        self.0
    }
    pub fn set_bit(self) -> &'a mut IntConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut IntConfigVal {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut IntConfigVal {
        self.0.0 &= !(1 << 0);
        self.0.0 |= (1 << 0) & 0x0;
        self.0
    }
}
