use crate::generic_config::ODR;
use crate::generic_config::DLPF;

#[derive(Copy, Clone, Debug)]
pub enum AccelRange {
    G16,
    G8,
    G4,
    G2,
}

impl Default for AccelRange {
    fn default() -> Self {
        Self::G16
    }
}

impl AccelRange {
    pub fn to_bits(self) -> u8{
        match self {
            Self::G16 => 0b00,
            Self::G8  => 0b01,
            Self::G4  => 0b10,
            Self::G2  => 0b11,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct AccelConfig {
    pub accel_range: AccelRange,
    pub accel_odr: ODR,
    pub accel_dlpf: DLPF,
}
