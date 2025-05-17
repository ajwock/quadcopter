use crate::imu_common::{
    Imu,
    ImuError,
    ImuMsg,
    ImuController,
};
use crate::motion_data::{MotionData, FixedMotionData, UnityFixed16, TiltData, RadianFixed16, to_unity, DegreeFixed16, DegreeFixed32};
use az::Cast;
use crate::debug_println;
use esp_println::println;
use fixed_macro::fixed;
use cordic::{
    cos,
    sin,
    acos,
    asin,
    atan2,
};
use fixed::types::I12F20;

pub fn euler_degrees_to_quaternion(
    roll: I12F20,
    pitch: I12F20,
    yaw: I12F20,
) -> (I12F20, I12F20, I12F20, I12F20) {
    const DEG_TO_RAD: I12F20 = fixed!(0.0174532925: I12F20); // π / 180
    let half = fixed!(0.5: I12F20);

    let r = roll * DEG_TO_RAD * half;
    let p = pitch * DEG_TO_RAD * half;
    let y = yaw * DEG_TO_RAD * half;

    let (cr, sr) = (cos(r), sin(r));
    let (cp, sp) = (cos(p), sin(p));
    let (cy, sy) = (cos(y), sin(y));

    let w = cr * cp * cy + sr * sp * sy;
    let x = sr * cp * cy - cr * sp * sy;
    let y = cr * sp * cy + sr * cp * sy;
    let z = cr * cp * sy - sr * sp * cy;

    (w, x, y, z)
}

const RAD_TO_DEG: I12F20 = fixed!(57.29578: I12F20); // 180 / π

pub fn quat_to_euler_degrees(w: I12F20, x: I12F20, y: I12F20, z: I12F20) -> (I12F20, I12F20, I12F20) {
    // roll (x-axis rotation)
    let sinr_cosp = fixed!(2.0: I12F20) * (w * x + y * z);
    let cosr_cosp = I12F20::ONE - fixed!(2.0: I12F20) * (x * x + y * y);
    let roll = atan2(sinr_cosp, cosr_cosp) * RAD_TO_DEG;

    // pitch (y-axis rotation)
    let sinp = fixed!(2.0: I12F20) * (w * y - z * x);
    let pitch = if sinp > I12F20::ONE {
        fixed!(90.0: I12F20)
    } else if sinp < -I12F20::ONE {
        fixed!(-90.0: I12F20)
    } else {
        asin(sinp) * RAD_TO_DEG
    };

    // yaw (z-axis rotation)
    let siny_cosp = fixed!(2.0: I12F20) * (w * z + x * y);
    let cosy_cosp = I12F20::ONE - fixed!(2.0: I12F20) * (y * y + z * z);
    let yaw = atan2(siny_cosp, cosy_cosp) * RAD_TO_DEG;

    (roll, pitch, yaw)
}
