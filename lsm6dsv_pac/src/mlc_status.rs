use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct MlcStatus<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> MlcStatus<'a, C> {
    pub fn read(&mut self) -> Result<MlcStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x15, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(MlcStatusVal(val))
    }
    pub async fn read_async(&mut self) -> Result<MlcStatusVal, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x15, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(MlcStatusVal(val))
    }
}
pub struct MlcStatusVal(pub u8);
impl MlcStatusVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn is_mlc8<'a>(&'a mut self) -> FieldIsMlc8<'a> {
        FieldIsMlc8(self)
    }
    pub fn is_mlc7<'a>(&'a mut self) -> FieldIsMlc7<'a> {
        FieldIsMlc7(self)
    }
    pub fn is_mlc6<'a>(&'a mut self) -> FieldIsMlc6<'a> {
        FieldIsMlc6(self)
    }
    pub fn is_mlc5<'a>(&'a mut self) -> FieldIsMlc5<'a> {
        FieldIsMlc5(self)
    }
    pub fn is_mlc4<'a>(&'a mut self) -> FieldIsMlc4<'a> {
        FieldIsMlc4(self)
    }
    pub fn is_mlc3<'a>(&'a mut self) -> FieldIsMlc3<'a> {
        FieldIsMlc3(self)
    }
    pub fn is_mlc2<'a>(&'a mut self) -> FieldIsMlc2<'a> {
        FieldIsMlc2(self)
    }
    pub fn is_mlc1<'a>(&'a mut self) -> FieldIsMlc1<'a> {
        FieldIsMlc1(self)
    }
}
pub struct FieldIsMlc8<'a>(pub &'a mut MlcStatusVal);
impl<'a> FieldIsMlc8<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsMlc7<'a>(pub &'a mut MlcStatusVal);
impl<'a> FieldIsMlc7<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsMlc6<'a>(pub &'a mut MlcStatusVal);
impl<'a> FieldIsMlc6<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsMlc5<'a>(pub &'a mut MlcStatusVal);
impl<'a> FieldIsMlc5<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsMlc4<'a>(pub &'a mut MlcStatusVal);
impl<'a> FieldIsMlc4<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsMlc3<'a>(pub &'a mut MlcStatusVal);
impl<'a> FieldIsMlc3<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsMlc2<'a>(pub &'a mut MlcStatusVal);
impl<'a> FieldIsMlc2<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldIsMlc1<'a>(pub &'a mut MlcStatusVal);
impl<'a> FieldIsMlc1<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
