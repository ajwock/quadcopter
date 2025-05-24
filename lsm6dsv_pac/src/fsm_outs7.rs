use core::result::Result;
use regcomms::{RegCommsError, RegComms, RegCommsAccessProc};
use crate::Lsm6Dsv;
pub struct FsmOuts7<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FsmOuts7<'a, C> {
    pub fn read(&mut self) -> Result<FsmOuts7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read(&mut self.0, 0x52, &mut buf)?;
        let val = u8::from_le_bytes(buf);
        Ok(FsmOuts7Val(val))
    }
    pub async fn read_async(&mut self) -> Result<FsmOuts7Val, RegCommsError> {
        let mut buf = [0u8; 1];
        let proc = self.0.embedded_func;
        proc.proc_read_async(&mut self.0, 0x52, &mut buf).await?;
        let val = u8::from_le_bytes(buf);
        Ok(FsmOuts7Val(val))
    }
}
pub struct FsmOuts7Val(pub u8);
impl FsmOuts7Val {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn p_x<'a>(&'a mut self) -> FieldPX<'a> {
        FieldPX(self)
    }
    pub fn n_x<'a>(&'a mut self) -> FieldNX<'a> {
        FieldNX(self)
    }
    pub fn p_y<'a>(&'a mut self) -> FieldPY<'a> {
        FieldPY(self)
    }
    pub fn n_y<'a>(&'a mut self) -> FieldNY<'a> {
        FieldNY(self)
    }
    pub fn p_z<'a>(&'a mut self) -> FieldPZ<'a> {
        FieldPZ(self)
    }
    pub fn n_z<'a>(&'a mut self) -> FieldNZ<'a> {
        FieldNZ(self)
    }
    pub fn p_v<'a>(&'a mut self) -> FieldPV<'a> {
        FieldPV(self)
    }
    pub fn n_v<'a>(&'a mut self) -> FieldNV<'a> {
        FieldNV(self)
    }
}
pub struct FieldPX<'a>(pub &'a mut FsmOuts7Val);
impl<'a> FieldPX<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldNX<'a>(pub &'a mut FsmOuts7Val);
impl<'a> FieldNX<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldPY<'a>(pub &'a mut FsmOuts7Val);
impl<'a> FieldPY<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldNY<'a>(pub &'a mut FsmOuts7Val);
impl<'a> FieldNY<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 4) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldPZ<'a>(pub &'a mut FsmOuts7Val);
impl<'a> FieldPZ<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldNZ<'a>(pub &'a mut FsmOuts7Val);
impl<'a> FieldNZ<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldPV<'a>(pub &'a mut FsmOuts7Val);
impl<'a> FieldPV<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 1) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
pub struct FieldNV<'a>(pub &'a mut FsmOuts7Val);
impl<'a> FieldNV<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 0) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
