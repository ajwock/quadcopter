use esp_hal::ledc;
use esp_hal::ledc::{
    channel::Channel,
};

type MotorChannel = Channel<'static, ledc::LowSpeed>;

// whoops sorry I guess
unsafe impl Send for Motor {}

pub(crate) struct Motor {
    pwm: MotorChannel,
}

impl Motor {
    fn new(pwm: MotorChannel) -> Self {
        Self {
            pwm,
        }
    }
}

// Motors are arranged geometrically for possibly intuitive understanding of this code?
pub(crate) struct MotorDrive {
    motors: [[Motor; 2]; 2],
}

impl MotorDrive {
    pub(crate) fn new(topleft: MotorChannel, topright: MotorChannel, bottomleft: MotorChannel, bottomright: MotorChannel) -> Self {
        Self {
            motors: [[Motor::new(topleft), Motor::new(topright)], [Motor::new(bottomleft), Motor::new(bottomright)]],
        }
    }
}
