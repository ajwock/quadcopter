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
use fixed_trigonometry::*;
use fixed::types::I12F20;

// This data structure represents integrated and fused IMU data to estimate
// orientation, speed, and position.
pub struct OrientationTracker<M: Imu> {
    pub orientation: [DegreeFixed32; 3],
    pub velocity: [DegreeFixed32; 3],
    pub position: [DegreeFixed32; 3],
    pub last_gyro_timestamp: u16,
    pub last_gyro_data_halved: [DegreeFixed32; 3],
    pub imuctl: ImuController<M>,
}

pub fn to_radians(degrees: I12F20) -> I12F20 {
    degrees * fixed!(0.0174532925: I12F20) // PI / 180
}

// Applies inverse rotation from orientation to accel vector
pub fn rotate_to_global_frame(
    accel: [I12F20; 3],
    orientation: [I12F20; 3],
) -> [I12F20; 3] {
    let [roll, pitch, yaw] = orientation.map(|i| to_radians(i));
    let sin_r = sin(-roll);
    let cos_r = cos(-roll);
    let sin_p = sin(-pitch);
    let cos_p = cos(-pitch);
    let sin_y = sin(-yaw);
    let cos_y = cos(-yaw);

    let [ax, ay, az] = accel;

    // Rotate X (roll)
    let ay1 = cos_r * ay - sin_r * az;
    let az1 = sin_r * ay + cos_r * az;

    // Rotate Y (pitch)
    let ax2 = cos_p * ax + sin_p * az1;
    let az2 = -sin_p * ax + cos_p * az1;

    // Rotate Z (yaw)
    let ax3 = cos_y * ax2 - sin_y * ay1;
    let ay3 = sin_y * ax2 + cos_y * ay1;

    [ax3, ay3, az2]
}

const ACCEL_SCALING_FACTOR: I12F20 = fixed!(0.000011962891: I12F20);
pub fn reading_to_accel_ms2(reading: i16) -> DegreeFixed32 {
    (reading as i32) * ACCEL_SCALING_FACTOR
}

// 2000/32767 (dps per lsb) * 1/100 * 1/100
// I still don't understand where the 100x error is coming from
const DPS_SCALING_FACTOR: I12F20 = fixed!(0.0000061037019: I12F20);
pub fn reading_to_dps_32_thousandth(reading: i16) -> I12F20 {
    (reading as i32) * DPS_SCALING_FACTOR
}

const HALVED_DPS_SCALING_FACTOR: I12F20 = fixed!(0.0000030518509: I12F20);
// Why this?  Saves us a separate muliply or division later on
pub fn halved_reading_to_dps_32(reading: i16) -> I12F20 {
    (reading as i32) * HALVED_DPS_SCALING_FACTOR 
}

// .000001 * 100
const TIME_SCALE_FACTOR: I12F20 = fixed!(0.0001: I12F20);
pub fn timestamp_diff_to_seconds(timestamp: u16) -> I12F20 {
    // Casting plus conversion from micros to seconds
    // I'm not entirely sure but I think this only works for U20 specifically
    (timestamp as i32) * TIME_SCALE_FACTOR
}

const DEGREES_180: DegreeFixed32 = DegreeFixed32::from_bits(188743680);
const DEGREES_NEG_180: DegreeFixed32 = DegreeFixed32::from_bits(-188743680);
pub fn degree_wrap(input: DegreeFixed32) -> DegreeFixed32 {
    if input < DEGREES_NEG_180 {
        let diff = DEGREES_NEG_180 - input;
        degree_wrap(DEGREES_180 - diff)
    } else if input > DEGREES_180 {
        let diff = input - DEGREES_180;
        degree_wrap(DEGREES_NEG_180 + diff)
    } else {
        input
    }
}

impl<M: Imu> OrientationTracker<M> {
    pub fn new(imuctl: ImuController<M>) -> Self {
        Self {
            orientation: [DegreeFixed32::from_bits(0); 3],
            velocity: [DegreeFixed32::from_bits(0); 3],
            position: [DegreeFixed32::from_bits(0); 3],
            last_gyro_timestamp: 0,
            last_gyro_data_halved: [DegreeFixed32::from_bits(0); 3],
            imuctl,
        }
    }

    pub fn get_orientation(&self) -> [DegreeFixed32; 3] {
        self.orientation
    }
   
    pub fn get_velocity(&self) -> [DegreeFixed32; 3] {
        self.velocity
    }

    pub fn get_position(&self) -> [DegreeFixed32; 3] {
        self.position
    }

    pub async fn track(&mut self) {
        let mut err_count = 0;
        loop {
            let msg = match self.imuctl.get_motion_data_msg().await {
                Ok(m) => m,
                Err(e) if e.is_not_ready() => {
                    break
                }
                Err(e) => {
                    println!("Orientation tracking got error: {:?}", e);
                    err_count += 1;
                    if err_count > 20 {
                        println!("Orientation tracking got too many errors...");
                        break
                    }
                    continue
                }
            };
            let timestamp = msg.timestamp;
            let accel_data = msg.accel_data;
            let gyro_data = msg.gyro_data;
            let timestamp_diff = if timestamp < self.last_gyro_timestamp {
                u16::MAX - self.last_gyro_timestamp + timestamp
            } else {
                self.last_gyro_timestamp - timestamp
            };
            let time_step = timestamp_diff_to_seconds(timestamp_diff);
            let accel_conv = accel_data.map(|i| reading_to_accel_ms2(i));
            let accel_objective = rotate_to_global_frame(accel_conv, self.orientation);

            // Doing riemann sum for accel, less need for precision
            for i in 0..3 {
                let delta_v = accel_objective[i] * time_step * fixed!(0.001: I12F20);
                self.velocity[i] += delta_v;
            }
/*
            for i in 0..3 {
                let delta_p = self.velocity[i] * time_step * fixed!(0.001: I12F20);
                self.position[i] += delta_p;
            }*/

            //  gyro data is in degrees per second, 16.4 LSB per degree/s
            //  timestamp in LSB per microsecond
            // Trapezoidal rule integration:
            // o_d is orientation in degrees, h is the time step between readings, o_d' is the output
            // of the gyroscope multiplied by degree per LSB:
            // o_d(t) = o_d(t - h) + h/2 * [o_d'(i - h) + o_d'(i)]
            // Below we pre-half our readings by lumping in a multiply by half with the
            // degree per lsb multiply.
            debug_println!("time_step: {time_step}");
            for i in 0..3 {
                let new_halved_reading = halved_reading_to_dps_32(gyro_data[i]);
                debug_println!("New_halved reading[{i}]: {new_halved_reading}");
                let avg_derivative = new_halved_reading + self.last_gyro_data_halved[i];
                let delta_orientation = avg_derivative * time_step;
                debug_println!("avg_derivative[{i}]: {avg_derivative}, delta orientation[{i}]: {delta_orientation}, last_gyro_data_halved[{i}]: {}", self.last_gyro_data_halved[i]);
                self.orientation[i] = self.orientation[i] + delta_orientation;
                self.last_gyro_data_halved[i] = new_halved_reading;
            }
            self.last_gyro_timestamp = timestamp;
        }
    }
}
