use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct CounterBdrReg1<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> CounterBdrReg1<'a, C> {
    pub fn read(&mut self) -> Result<CounterBdrReg1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read(&mut self.0, 0xb, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(CounterBdrReg1Val(val))
    }
    pub async fn read_async(&mut self) -> Result<CounterBdrReg1Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.standard;
        proc.proc_read_async(&mut self.0, 0xb, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(CounterBdrReg1Val(val))
    }
    pub fn write(&mut self, val: CounterBdrReg1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write(&mut self.0, 0xb, &buf)?;
        Ok(())
    }
    pub fn write_raw(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write(CounterBdrReg1Val(raw_val))
    }
    pub async fn write_async(&mut self, val: CounterBdrReg1Val) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        let proc = self.0.standard;
        proc.proc_write_async(&mut self.0, 0xb, &buf).await?;
        Ok(())
    }
    pub async fn write_raw_async(&mut self, raw_val: u8) -> Result<(), RegCommsError> {
        self.write_async(CounterBdrReg1Val(raw_val)).await
    }
    pub fn modify<F: FnOnce(CounterBdrReg1Val) -> CounterBdrReg1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read()?;
        self.write(f(orig_val))
    }
    pub async fn modify_async<F: FnOnce(CounterBdrReg1Val) -> CounterBdrReg1Val>(&mut self, f: F) -> Result<(), RegCommsError> {
        let orig_val = self.read_async().await?;
        self.write_async(f(orig_val)).await
    }
    pub fn reset(&mut self) -> Result<(), RegCommsError> {
        self.write(CounterBdrReg1Val(0x0))
    }
    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {
        self.write_async(CounterBdrReg1Val(0x0)).await
    }
}
pub struct CounterBdrReg1Val(pub u8);
impl CounterBdrReg1Val {
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
    pub fn trig_counter_bdr<'a>(&'a mut self) -> FieldTrigCounterBdr<'a> {
        FieldTrigCounterBdr(self)
    }
    pub fn xl_hg_batch_en<'a>(&'a mut self) -> FieldXlHgBatchEn<'a> {
        FieldXlHgBatchEn(self)
    }
    pub fn cnt_bdr_th_9_8<'a>(&'a mut self) -> FieldCntBdrTh98<'a> {
        FieldCntBdrTh98(self)
    }
}
pub struct FieldTrigCounterBdr<'a>(pub &'a mut CounterBdrReg1Val);
impl<'a> FieldTrigCounterBdr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut CounterBdrReg1Val {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 5;
        self.0
    }
    pub fn reset(self) -> &'a mut CounterBdrReg1Val {
        self.0.0 &= !(!(!0 << 2) << 5);
        self.0.0 |= 0x0 & (!(!0 << 2) << 5);
        self.0
    }
}
pub struct FieldXlHgBatchEn<'a>(pub &'a mut CounterBdrReg1Val);
impl<'a> FieldXlHgBatchEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut CounterBdrReg1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (val as u8) << 4;
        self.0
    }
    pub fn set_bit(self) -> &'a mut CounterBdrReg1Val {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut CounterBdrReg1Val {
        self.assign(false)
    }
    pub fn reset(self) -> &'a mut CounterBdrReg1Val {
        self.0.0 &= !(1 << 4);
        self.0.0 |= (1 << 4) & 0x0;
        self.0
    }
}
pub struct FieldCntBdrTh98<'a>(pub &'a mut CounterBdrReg1Val);
impl<'a> FieldCntBdrTh98<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut CounterBdrReg1Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
    pub fn reset(self) -> &'a mut CounterBdrReg1Val {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= 0x0 & (!(!0 << 2) << 0);
        self.0
    }
}
