use crate::accel_config::AccelConfig;
use crate::gyro_config::GyroConfig;
use crate::fifo_config::FifoConfig;

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub accel_config: Option<AccelConfig>,
    pub gyro_config:  Option<GyroConfig>,
    pub fifo_config:  Option<FifoConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            accel_config: Some(Default::default()),
            gyro_config:  Some(Default::default()),
            fifo_config:  None,
        }
    }
}
