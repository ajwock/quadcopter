use fixed::{FixedI16, FixedI32};
use esp_println::println;
use core::ops::{Add, Sub, Mul, Div, Shl, Shr};
use fixed::types::extra::{U8, U15, U12};
use crate::debug_println;
use crate::motion_data::MotionData;
use fixed_trigonometry::{
    atan::atan2,
    wrap_phase,
};
pub type UnityFixed16 = FixedI16<U15>;
pub type RadianFixed16 = FixedI16<U12>;

pub(crate) struct RPYMotionData {
    accel_roll: RadianFixed16,
    accel_pitch: RadianFixed16,
    accel_yaw: RadianFixed16,
    gyro_roll_ddt: RadianFixed16,
    gyro_pitch_ddt: RadianFixed16,
    gyro_yaw_ddt: RadianFixed16,
}

impl From<MotionData> for RPYMotionData {
    fn from(other: MotionData) -> Self {

        todo!()
    }
}
