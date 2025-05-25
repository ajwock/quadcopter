use crate::imu_common::{
    Imu,
    ImuController,
};
use crate::motion_data::DegreeFixed32;
use crate::debug_println;
use esp_println::println;
use fixed_macro::fixed;
use fixed_trigonometry::atan::atan2;
use fixed::types::I12F20;

// This data structure represents integrated and fused IMU data to estimate
// orientation, speed, and position.
pub struct OrientationTracker<M: Imu> {
    pub orientation: [DegreeFixed32; 3],
    pub fused_orientation: [DegreeFixed32; 3],
    pub accel_tilt: [DegreeFixed32; 2],
    pub last_gyro_timestamp: u16,
    pub last_gyro_data_halved: [DegreeFixed32; 3],
    pub imuctl: ImuController<M>,
}

const ACCEL_SCALING_FACTOR: I12F20 = fixed!(0.0011962891: I12F20);
pub fn reading_to_accel_ms2(reading: i16) -> DegreeFixed32 {
    (reading as i32) * ACCEL_SCALING_FACTOR
}

// Why this?  Saves us a separate muliply or division later on
const HALVED_DEGREE_SCALING_FACTOR: I12F20 = fixed!(0.0000030518509: I12F20);
pub fn halved_reading_to_dps_32(reading: i16) -> DegreeFixed32 {
    (reading as i32) * HALVED_DEGREE_SCALING_FACTOR
}

// .000001 * 100
const TIME_SCALE_FACTOR: I12F20 = fixed!(0.0001: I12F20);
pub fn timestamp_diff_to_seconds(timestamp: u16) -> I12F20 {
    // Casting plus conversion from micros to seconds
    // I'm not entirely sure but I think this only works for U20 specifically
    (timestamp as i32) * TIME_SCALE_FACTOR
}

const FRAC_180_PI: DegreeFixed32 = fixed!(57.29577: I12F20);

impl<M: Imu> OrientationTracker<M> {
    pub fn new(imuctl: ImuController<M>) -> Self {
        Self {
            orientation: [DegreeFixed32::from_bits(0); 3],
            fused_orientation: [DegreeFixed32::from_bits(0); 3],
            accel_tilt: Default::default(),
            last_gyro_timestamp: 0,
            last_gyro_data_halved: [DegreeFixed32::from_bits(0); 3],
            imuctl,
        }
    }

    pub fn get_orientation(&self) -> [DegreeFixed32; 3] {
        self.orientation
    }

    const COMPLEMENTARY_ALPHA: DegreeFixed32 = fixed!(0.996: I12F20);
    pub fn complementary_filter(gyro_degrees: I12F20, accel_degrees: I12F20) -> I12F20 {
        Self::COMPLEMENTARY_ALPHA * gyro_degrees + (I12F20::ONE - Self::COMPLEMENTARY_ALPHA) * accel_degrees
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
                    debug_println!("Orientation tracking got error: {:?}", e);
                    err_count += 1;
                    if err_count > 20 {
                        panic!("Orientation tracking got too many errors...");
                    }
                    continue
                }
            };
            let timestamp = msg.timestamp;
            let accel_data = msg.accel_data;
            let gyro_data = msg.gyro_data;

            let [acc_x, acc_y, acc_z] = accel_data.map(|elt| reading_to_accel_ms2(elt) * fixed!(0.1: I12F20));
            // Oh... I'm now aware of so much wrong I've been doing to myself...
            // Maybe I need to explicitly use the terms pitch and roll.  TODO
            //
            let att_acc_y = atan2(-acc_x, (acc_y*acc_y + acc_z*acc_z).sqrt()) * FRAC_180_PI;
            //let att_acc_y = -Self::att_accel(acc_z, acc_x);
            let att_acc_x = atan2(acc_y, (acc_x*acc_x + acc_z*acc_z).sqrt()) * FRAC_180_PI;
//            let att_acc_x = Self::att_accel(acc_z, acc_y);
            self.accel_tilt = [att_acc_x, att_acc_y];
/*            if gyro_data[0] as u16 == 0xffff || gyro_data[1] as u16 == 0xffff || gyro_data[2] as u16 == 0xffff {
                panic!("Got erronious gyro data value");
            }*/
            let timestamp_diff = if timestamp < self.last_gyro_timestamp {
                u16::MAX - self.last_gyro_timestamp + timestamp
            } else {
                self.last_gyro_timestamp - timestamp
            };

            //  gyro data is in degrees per second, 16.4 LSB per degree/s
            //  timestamp in LSB per microsecond
            // Trapezoidal rule integration:
            // o_d is orientation in degrees, h is the time step between readings, o_d' is the output
            // of the gyroscope multiplied by degree per LSB:
            // o_d(t) = o_d(t - h) + h/2 * [o_d'(i - h) + o_d'(i)]
            // Below we pre-half our readings by lumping in a multiply by half with the
            // degree per lsb multiply.
            let time_step = timestamp_diff_to_seconds(timestamp_diff);
            debug_println!("time_step: {time_step}");
            for i in 0..3 {
                let new_halved_reading = halved_reading_to_dps_32(gyro_data[i]);
                debug_println!("New_halved reading[{i}]: {new_halved_reading}");
                let avg_derivative = new_halved_reading + self.last_gyro_data_halved[i];
                let delta_orientation = avg_derivative * time_step;
                debug_println!("avg_derivative[{i}]: {avg_derivative}, delta orientation[{i}]: {delta_orientation}, last_gyro_data_halved[{i}]: {}", self.last_gyro_data_halved[i]);
                self.orientation[i] = self.orientation[i] + delta_orientation;
                self.fused_orientation[i] = self.fused_orientation[i] + delta_orientation;
                self.last_gyro_data_halved[i] = new_halved_reading;
            }
            self.last_gyro_timestamp = timestamp;
        }
        self.fused_orientation[0] = Self::complementary_filter(self.fused_orientation[0], self.accel_tilt[0]);
        self.fused_orientation[1] = Self::complementary_filter(self.fused_orientation[1], self.accel_tilt[1]);
        self.orientation = self.fused_orientation;
    }
}
