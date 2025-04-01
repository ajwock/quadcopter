use esp_hal::ledc;
use esp_hal::ledc::{
    channel::{Channel, ChannelIFace},
};
use crate::debug_println;
use esp_println::println;
use crate::motion_data::{MotionData, FixedMotionData, UnityFixed16, TiltData, RadianFixed16, to_unity};
use fixed::FixedI16;
use fixed::types::extra::U8;
use fixed_trigonometry::powi;
use az::Cast;
use crate::utils;

// (1 - x)^31
// Even a small amount of error makes accelerometer a less than trustworth source of
// orientation data, but when it's equal to the gravity magnitude that's perfect.
const ACCEL_TRUST_FN: [i16; 1024] = [32737, 31776, 30856, 29962, 29092, 28247, 27430, 26630, 25853, 25102, 24368, 23655, 22965, 22291, 21640, 21008, 20390, 19791, 19209, 18643, 18095, 17562, 17045, 16541, 16050, 15577, 15116, 14666, 14232, 13808, 13400, 12999, 12612, 12238, 11871, 11516, 11171, 10840, 10511, 10197, 9890, 9594, 9305, 9025, 8753, 8489, 8233, 7983, 7741, 7507, 7280, 7060, 6846, 6637, 6433, 6238, 6049, 5863, 5683, 5510, 5342, 5177, 5019, 4865, 4716, 4571, 4431, 4292, 4159, 4031, 3906, 3784, 3668, 3555, 3444, 3337, 3233, 3132, 3035, 2940, 2847, 2758, 2674, 2589, 2508, 2427, 2352, 2280, 2206, 2137, 2069, 2005, 1939, 1878, 1819, 1761, 1706, 1652, 1598, 1547, 1498, 1450, 1404, 1359, 1316, 1272, 1233, 1191, 1153, 1118, 1081, 1045, 1012, 979, 947, 917, 886, 857, 830, 803, 777, 751, 727, 703, 678, 657, 635, 616, 594, 575, 556, 537, 519, 503, 486, 470, 454, 439, 424, 409, 397, 383, 371, 357, 345, 334, 323, 312, 300, 292, 281, 271, 263, 254, 245, 237, 228, 221, 212, 206, 199, 192, 186, 179, 173, 167, 161, 156, 149, 145, 140, 134, 129, 126, 121, 117, 112, 109, 105, 101, 97, 95, 91, 87, 84, 80, 79, 76, 73, 70, 68, 66, 63, 60, 57, 57, 54, 52, 51, 48, 47, 45, 43, 41, 40, 38, 37, 36, 34, 32, 32, 31, 29, 27, 27, 27, 25, 23, 23, 22, 21, 20, 20, 19, 18, 17, 17, 16, 14, 14, 14, 13, 13, 13, 12, 11, 11, 10, 10, 10, 10, 9, 8, 8, 7, 7, 6, 6, 6, 6, 6, 6, 5, 5, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn accelerometer_trust(x: UnityFixed16) -> UnityFixed16 {
    let x = x.abs().to_bits();
    let index = (x / 32) as usize;
    println!("x bits: {x}, index: {index}");
    UnityFixed16::from_bits(ACCEL_TRUST_FN[index])
}

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
    motor_targets: [[u8; 2]; 2],
    current_drive: [[u8; 2]; 2],
    attitude_int: [UnityFixed16; 2],
    gravity_magnitude: RadianFixed16,
    collective_power: u16,
    // Desired tilt and gyre vector, for lateral movement and rotation.
    // Please pre-normalize!
    target_tilt: [RadianFixed16; 2],
    previous_acc_error: [UnityFixed16; 3],
    integrated_acc_error: [UnityFixed16; 3],
}


impl MotorDrive {
    pub(crate) fn new(topleft: MotorChannel, topright: MotorChannel, bottomleft: MotorChannel, bottomright: MotorChannel, gravity_magnitude: i16) -> Self {
        Self {
            motors: [[Motor::new(topleft), Motor::new(topright)], [Motor::new(bottomleft), Motor::new(bottomright)]],
            motor_targets: [[0; 2]; 2],
            current_drive: [[0; 2]; 2],
            attitude_int: [UnityFixed16::from_bits(0); 2],
            collective_power: 16535,
            target_tilt: Default::default(),
            previous_acc_error: Default::default(),
            integrated_acc_error: Default::default(),
            gravity_magnitude: RadianFixed16::from_bits(gravity_magnitude),
        }
    }

    const SLEW_CONSTANT: u8 = 2;
    // Call this with timed ticks to apply motor slew
    pub(crate) fn motor_tick(&mut self) {
        for r in 0..2 {
            for c in 0..2 {
                self.current_drive[r][c] = utils::rate_limit(self.current_drive[r][c], self.motor_targets[r][c], Self::SLEW_CONSTANT);
                debug_println!("setting motor[{}][{}] duty to {}", r, c, self.current_drive[r][c]);
                self.motors[r][c].pwm.set_duty(self.current_drive[r][c])
                    .unwrap();
            }
        }
    }

    pub(crate) fn set_collective_pct(&mut self, pct: u8) {
        const MAX_INPUT: u32 = 100;
        const MAX_OUTPUT: u32 = i16::MAX as u32;
        const RATIO: u32 = MAX_OUTPUT / MAX_INPUT;
        let pct_clamped = core::cmp::min(pct, 100);
        let power_val = pct_clamped as u32 * RATIO;
        self.collective_power = power_val as u16;
        debug_println!("Setting collective: {}", self.collective_power);
    }

    // PID constants.
    const ATTITUDE_POSITION: UnityFixed16 = UnityFixed16::from_bits(i16::MAX);
    const ATTITUDE_INTEGRAL: UnityFixed16 = UnityFixed16::from_bits(0);
    const ATTITUDE_INTEGRAL_PERTICK: UnityFixed16 = UnityFixed16::from_bits(i16::MAX / 32);

    const ROTATION_POSITION: UnityFixed16 = UnityFixed16::from_bits(i16::MAX);
    pub(crate) fn attitude_correct(&mut self, data: MotionData) {
        //let fdata: FixedMotionData = data.into();

        // Handle acceleration adjustments
//        let acc_v = fdata.normalized_acc();
        let orientation_data: TiltData = data.into();
        let tilt_v = orientation_data.tilt_vector();
        debug_println!("xz_tilt, yz_tilt: [{}, {}]", tilt_v[0], tilt_v[1]);
        debug_println!("accel_magnitude: {}", orientation_data.accel_magnitude);
        let mag = orientation_data.accel_magnitude;
        let grav_diff = self.gravity_magnitude.saturating_sub(mag).abs();
        println!("Gravity_mag vs unity mag: {} vs {}", self.gravity_magnitude, mag);
        println!("Grav diff: {grav_diff}");
        let acc_trust = accelerometer_trust(to_unity(grav_diff));
        debug_println!("Accelerometer trust: {acc_trust}");
        // Uhhh do we wanna do trig wrap here
        let err_v_rad: [_; 2] = core::array::from_fn(|i| self.target_tilt[i] - tilt_v[i]);
        let err_v = err_v_rad.map(|x| to_unity(x));
        // Position handling
        //debug_println!("acc_v: {:?}", acc_v);
        //debug_println!("err_v: {:?}", err_v);
        let mut motor_adjustments = [[UnityFixed16::from_num(0); 2]; 2];
        // Motors get power added if craft is tilting towards either of the 4 rectangular edges
        // that the motor sits at the corner to.
                // Attitude integral error handling
        self.attitude_int = core::array::from_fn(|i| self.attitude_int[i] + err_v[i] * Self::ATTITUDE_INTEGRAL_PERTICK);
        debug_println!("Attitude integral: {:?}", self.attitude_int);

        let adj_fn: [_; 2]  = core::array::from_fn(|i| err_v[i] * Self::ATTITUDE_POSITION + self.attitude_int[i] * Self::ATTITUDE_INTEGRAL);

        motor_adjustments[0][0] = motor_adjustments[0][0].saturating_add(adj_fn[0]).saturating_add(adj_fn[1]);
        motor_adjustments[0][1] = motor_adjustments[0][1].saturating_add(adj_fn[0]).saturating_sub(adj_fn[1]);
        motor_adjustments[1][0] = motor_adjustments[1][0].saturating_sub(adj_fn[0]).saturating_add(adj_fn[1]);
        motor_adjustments[1][1] = motor_adjustments[1][1].saturating_sub(adj_fn[0]).saturating_sub(adj_fn[1]);

        // Handle gyro adjustments.  Only concerned with rotation about z right now as attitude
        // corrections should handle xy rotation
        let rotation_error = UnityFixed16::from_bits(data.gyr_z);
        let rot_fn = rotation_error * Self::ROTATION_POSITION;
        // Opposite motors have propellers rotating in opposite directions.
        // The rotation error is added to or subtracted from opposite propellers to create torque
        // in one direction about the z axis without significantly influencing attitude.
        motor_adjustments[0][0] = motor_adjustments[0][0].saturating_sub(rot_fn);
        motor_adjustments[0][1] = motor_adjustments[0][1].saturating_add(rot_fn);
        motor_adjustments[1][0] = motor_adjustments[1][0].saturating_add(rot_fn);
        motor_adjustments[1][1] = motor_adjustments[1][1].saturating_sub(rot_fn);


        let mut scalers = [[UnityFixed16::from_bits(self.collective_power as i16); 2]; 2];
        for i in 0..scalers[0].len() {
            for j in 0..scalers[0].len() {
                scalers[i][j] = scalers[i][j].saturating_add(motor_adjustments[i][j]).max(UnityFixed16::ZERO);
            }
        }
       debug_println!("scalers: {:?}", scalers);
        for i in 0..scalers[0].len() {
            for j in 0..scalers[1].len() {
                let s_cast = Cast::<FixedI16<U8>>::cast(scalers[i][j]);
                let duty_fixed = s_cast * 100;
                let duty = Cast::<u8>::cast(duty_fixed);
                self.motor_targets[i][j] = duty;
            }
        }
    }
}
