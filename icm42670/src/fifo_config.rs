#[derive(Copy, Clone, Debug)]
pub enum FifoMode {
    Stream,
    StopOnFull,
}

impl Default for FifoMode {
    fn default() -> Self {
        Self::Stream
    }
}

impl FifoMode {
    pub fn to_bit(self) -> bool {
        match self {
            Self::Stream => false,
            Self::StopOnFull => true,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FifoConfig {
    pub watermark: Option<u16>,
    pub mode: FifoMode,
}
