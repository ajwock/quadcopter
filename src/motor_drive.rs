use esp_hal::ledc;
use esp_hal::ledc::{
    channel::{Channel, ChannelIFace},
};
use crate::debug_println;
use esp_println::println;
use crate::motion_data::{MotionData, FixedMotionData, UnityFixed16, TiltData, RadianFixed16, to_unity, DegreeFixed32};
use fixed::FixedI16;
use fixed::types::extra::U8;
use fixed_macro::fixed;
use fixed_trigonometry::powi;
use az::Cast;
use crate::utils;
use crate::delay_buf::DelayBuf;

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
    attitude_int: [DegreeFixed32; 3],
    gravity_magnitude: RadianFixed16,
    collective_power: DegreeFixed32,
    // Desired tilt and gyre vector, for lateral movement and rotation.
    // Please pre-normalize!
    target_tilt: [DegreeFixed32; 3],
    // For orientation-based derivative factor
    previous_orientation: [DegreeFixed32; 3],
    target_tilt_derivative_buf: DelayBuf<[DegreeFixed32; 3], 4>,
    previous_acc_error: [UnityFixed16; 3],
    integrated_acc_error: [UnityFixed16; 3],
}

impl MotorDrive {
    pub(crate) fn new(topleft: MotorChannel, topright: MotorChannel, bottomleft: MotorChannel, bottomright: MotorChannel, gravity_magnitude: i16) -> Self {
        Self {
            motors: [[Motor::new(topleft), Motor::new(topright)], [Motor::new(bottomleft), Motor::new(bottomright)]],
            motor_targets: [[0; 2]; 2],
            current_drive: [[0; 2]; 2],
            attitude_int: [DegreeFixed32::from_bits(0); 3],
            collective_power: DegreeFixed32::from_bits(0),
            target_tilt: Default::default(),
            target_tilt_derivative_buf: DelayBuf::new_with_default(Default::default()),
            previous_orientation: Default::default(),
            previous_acc_error: Default::default(),
            integrated_acc_error: Default::default(),
            gravity_magnitude: RadianFixed16::from_bits(gravity_magnitude),
        }
    }

    const SLEW_CONSTANT_UP: u8 = 10;
    const SLEW_CONSTANT_DOWN: u8 = 10;
    // Call this with timed ticks to apply motor slew
    pub(crate) fn motor_tick(&mut self) {
        for r in 0..2 {
            for c in 0..2 {
                self.current_drive[r][c] = utils::asymmetrical_rate_limit(self.current_drive[r][c], self.motor_targets[r][c], Self::SLEW_CONSTANT_UP, Self::SLEW_CONSTANT_DOWN);
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
        self.collective_power = DegreeFixed32::from_num(pct_clamped) / 100;
        debug_println!("Setting collective: {}", self.collective_power);
    }

    // PID constants.
    const ATTITUDE_POSITION: DegreeFixed32 = fixed!(0.3: I12F20); // fixed!(0.5: I12F20);
    const POSITION_CLAMP: DegreeFixed32 = fixed!(0.1: I12F20);
    const ATTITUDE_INTEGRAL: DegreeFixed32 = DegreeFixed32::from_bits(0);
    const ATTITUDE_INTEGRAL_PERTICK: DegreeFixed32 = DegreeFixed32::from_bits(0);
    const INTEGRAL_CLAMP: DegreeFixed32 = fixed!(0.05: I12F20);
    const ATTITUDE_DERIVATIVE: DegreeFixed32 = fixed!(5: I12F20);
    const DERIVATIVE_CLAMP: DegreeFixed32 = fixed!(0.07: I12F20);
    const ROTATION_POSITION: DegreeFixed32 = fixed!(0.25: I12F20);
    pub(crate) fn attitude_correct_2(&mut self, ) {
        
    }
    pub(crate) fn attitude_correct(&mut self, data: [DegreeFixed32; 3]) {
        //let fdata: FixedMotionData = data.into();

        // Handle acceleration adjustments
        let tilt_v = data.map(|x| x / 180);
        debug_println!("xz_tilt, yz_tilt: [{}, {}]", tilt_v[0], tilt_v[1]);
//        debug_println!("accel_magnitude: {}", orientation_data.accel_magnitude);
 //       let mag = orientation_data.accel_magnitude;
  //      let grav_diff = self.gravity_magnitude.saturating_sub(mag).abs();
  //      println!("Gravity_mag vs unity mag: {} vs {}", self.gravity_magnitude, mag);
 //       println!("Grav diff: {grav_diff}");
        // Uhhh do we wanna do trig wrap here
       let err_v: [_; 3] = core::array::from_fn(|i| self.target_tilt[i] - tilt_v[i]);
        // Position handling
        //debug_println!("acc_v: {:?}", acc_v);
        //debug_println!("err_v: {:?}", err_v);
        let mut motor_adjustments = [[DegreeFixed32::from_num(0); 2]; 2];
        // Motors get power added if craft is tilting towards either of the 4 rectangular edges
        // that the motor sits at the corner to.
                // Attitude integral error handling
        self.attitude_int = core::array::from_fn(|i| self.attitude_int[i] + err_v[i] * Self::ATTITUDE_INTEGRAL_PERTICK);
        debug_println!("Attitude integral: {:?}", self.attitude_int);

        let derivative: [_; 3] = core::array::from_fn(|i| 100 * (self.previous_orientation[i] - tilt_v[i]));
        self.previous_orientation = tilt_v;

        let adj_fn: [_; 3]  = core::array::from_fn(|i| (err_v[i] * Self::ATTITUDE_POSITION).clamp(-Self::POSITION_CLAMP, Self::POSITION_CLAMP) + (self.attitude_int[i] * Self::ATTITUDE_INTEGRAL).clamp(-Self::INTEGRAL_CLAMP, Self::INTEGRAL_CLAMP) + (derivative[i] * Self::ATTITUDE_DERIVATIVE).clamp(-Self::DERIVATIVE_CLAMP, Self::DERIVATIVE_CLAMP));

        // Frontleft is in the -x, +y region
        motor_adjustments[0][0] = motor_adjustments[0][0].saturating_add(adj_fn[0]).saturating_sub(adj_fn[1]);
        motor_adjustments[0][1] = motor_adjustments[0][1].saturating_sub(adj_fn[0]).saturating_sub(adj_fn[1]);
        motor_adjustments[1][0] = motor_adjustments[1][0].saturating_add(adj_fn[0]).saturating_add(adj_fn[1]);
        motor_adjustments[1][1] = motor_adjustments[1][1].saturating_sub(adj_fn[0]).saturating_add(adj_fn[1]);

        // Handle gyro adjustments.  Only concerned with rotation about z right now as attitude
        // corrections should handle xy rotation
        let rotation_error = err_v[2];
        let rot_fn = rotation_error * Self::ROTATION_POSITION;
        // Opposite motors have propellers rotating in opposite directions.
        // The rotation error is added to or subtracted from opposite propellers to create torque
        // in one direction about the z axis without significantly influencing attitude.
        motor_adjustments[0][0] = motor_adjustments[0][0].saturating_sub(rot_fn);
        motor_adjustments[0][1] = motor_adjustments[0][1].saturating_add(rot_fn);
        motor_adjustments[1][0] = motor_adjustments[1][0].saturating_add(rot_fn);
        motor_adjustments[1][1] = motor_adjustments[1][1].saturating_sub(rot_fn);


        let mut scalers = [[self.collective_power; 2]; 2];
        for i in 0..scalers[0].len() {
            for j in 0..scalers[0].len() {
                scalers[i][j] = scalers[i][j].saturating_add(motor_adjustments[i][j]).clamp(DegreeFixed32::ZERO, DegreeFixed32::ONE);
            }
        }
       println!("scalers: {:?}", scalers);
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
