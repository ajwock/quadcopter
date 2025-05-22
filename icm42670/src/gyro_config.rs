use crate::generic_config::ODR;
use crate::generic_config::DLPF;

#[derive(Copy, Clone, Debug)]
pub enum GyroRange {
    DPS2000,
    DPS1000,
    DPS500,
    DPS250,
}

impl Default for GyroRange {
    fn default() -> Self {
        Self::DPS2000
    }
}

impl GyroRange {
    pub fn to_bits(self) -> u8 {
        match self {
            Self::DPS2000 => 0b00,
            Self::DPS1000 => 0b01,
            Self::DPS500  => 0b10,
            Self::DPS250  => 0b11,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct GyroConfig {
    pub gyro_range: GyroRange,
    pub gyro_odr: ODR,
    pub gyro_dlpf: DLPF,
}
