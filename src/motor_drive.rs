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
            collective_power: 16535,
            target_acc_vector: Default::default(),
            previous_acc_error: Default::default(),
            integrated_acc_error: Default::default(),
        }
    }

    pub(crate) fn set_collective_pct(&mut self, pct: u8) {
        const MAX_INPUT: u32 = 100;
        const MAX_OUTPUT: u32 = i16::MAX as u32;
        const RATIO: u32 = MAX_OUTPUT / MAX_INPUT;
        let pct_clamped = core::cmp::min(pct, 100);
        let power_val = pct_clamped as u32 * RATIO;
        self.collective_power = power_val as u16;
        println!("Setting collective: {}", self.collective_power);
    }

    pub(crate) fn attitude_correct(&mut self, data: MotionData) {
        let fdata: FixedMotionData = data.into();

        // Handle acceleration adjustments
        let acc_v = fdata.normalized_acc();
        let err_v: [_; 3] = core::array::from_fn(|i| self.target_acc_vector[i] - acc_v[i]);
        println!("acc_v: {:?}", acc_v);
        println!("err_v: {:?}", err_v);
        let mut motor_adjustments = [[UnityFixed16::from_num(0); 2]; 2];
        motor_adjustments[0][0] = motor_adjustments[0][0].saturating_add(err_v[0]).saturating_add(err_v[1]);
        motor_adjustments[0][1] = motor_adjustments[0][1].saturating_add(err_v[0]).saturating_sub(err_v[1]);
        motor_adjustments[1][0] = motor_adjustments[1][0].saturating_sub(err_v[0]).saturating_add(err_v[1]);
        motor_adjustments[1][1] = motor_adjustments[1][1].saturating_sub(err_v[0]).saturating_sub(err_v[1]);


        
        let mut scalers = [[UnityFixed16::from_bits(self.collective_power as i16); 2]; 2];
        for i in 0..scalers[0].len() {
            for j in 0..scalers[0].len() {
                scalers[i][j] = scalers[i][j].saturating_add(motor_adjustments[i][j]).max(UnityFixed16::ZERO);
            }
        }
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
