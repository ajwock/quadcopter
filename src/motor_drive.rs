use esp_hal::ledc;
use esp_hal::ledc::{
    channel::{Channel, ChannelIFace},
};
use esp_println::println;
use crate::motion_data::{MotionData, FixedMotionData, UnityFixed16};
use fixed::FixedI16;
use fixed::types::extra::U8;
use az::Cast;

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
    collective_power: u16,
    // Desired tilt and gyre vector, for lateral movement and rotation.
    // Please pre-normalize!
    target_acc_vector: [UnityFixed16; 3],
    previous_acc_error: [UnityFixed16; 3],
    integrated_acc_error: [UnityFixed16; 3],
}

impl MotorDrive {
    pub(crate) fn new(topleft: MotorChannel, topright: MotorChannel, bottomleft: MotorChannel, bottomright: MotorChannel) -> Self {
        Self {
            motors: [[Motor::new(topleft), Motor::new(topright)], [Motor::new(bottomleft), Motor::new(bottomright)]],
            collective_power: 0,
            target_acc_vector: Default::default(),
            previous_acc_error: Default::default(),
            integrated_acc_error: Default::default(),
        }
    }

    pub(crate) fn attitude_correct(&mut self, data: MotionData) {
        let fdata: FixedMotionData = data.into();
        let acc_v = fdata.normalized_acc();
        let err_v: [_; 3] = core::array::from_fn(|i| self.target_acc_vector[i] - acc_v[i]);
        println!("acc_v: {:?}", acc_v);
        println!("err_v: {:?}", err_v);
        let mut scalers = [[UnityFixed16::from_bits(self.collective_power as i16); 2]; 2];
        scalers[0][0] = scalers[0][0].saturating_sub(err_v[0]).saturating_sub(err_v[1]).max(UnityFixed16::ZERO);
        scalers[0][1] = scalers[0][1].saturating_sub(err_v[0]).saturating_add(err_v[1]).max(UnityFixed16::ZERO);
        scalers[1][0] = scalers[1][0].saturating_add(err_v[0]).saturating_sub(err_v[1]).max(UnityFixed16::ZERO);
        scalers[1][1] = scalers[1][1].saturating_add(err_v[0]).saturating_add(err_v[1]).max(UnityFixed16::ZERO);
        println!("scalers: {:?}", scalers);
        for i in 0..scalers[0].len() {
            for j in 0..scalers[1].len() {
                let s_cast = Cast::<FixedI16<U8>>::cast(scalers[i][j]);
                let duty_fixed = s_cast * 100;
                let duty = Cast::<u8>::cast(duty_fixed);
                println!("setting motor[{}][{}] duty to {}", i, j, duty);
                self.motors[i][j].pwm.set_duty(duty)
                    .unwrap();
            }
        }
    }
}
