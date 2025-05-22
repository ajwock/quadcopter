#[derive(Copy, Clone, Debug)]
pub enum DLPF {
    Bypassed,
    Hz180,
    Hz121,
    Hz73,
    Hz53,
    Hz34,
    Hz25,
    Hz16,
}

impl Default for DLPF {
    fn default() -> Self {
        Self::Bypassed
    }
}

impl DLPF {
    pub fn to_bits(self) -> u8 {
        match self {
            Self::Bypassed => 0b000,
            Self::Hz180 =>    0b001,
            Self::Hz121 =>    0b010,
            Self::Hz73 =>     0b011,
            Self::Hz53 =>     0b100,
            Self::Hz34 =>     0b101,
            Self::Hz25 =>     0b110,
            Self::Hz16 =>     0b111,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ODR {
    Hz1600,
    Hz800,
    Hz400,
    Hz200,
    Hz100,
    Hz50,
    Hz25,
    Hz12_5,
}

impl Default for ODR {
    fn default() -> Self {
        Self::Hz1600
    }
}

impl ODR {
    pub fn to_bits(self) -> u8 {
        match self {
            Self::Hz1600 => 0b0101,
            Self::Hz800 =>  0b0110,
            Self::Hz400 =>  0b0111,
            Self::Hz200 =>  0b1000,
            Self::Hz100 =>  0b1001,
            Self::Hz50  =>  0b1010,
            Self::Hz25  =>  0b1011,
            Self::Hz12_5 => 0b1100,
       }
    }
}
