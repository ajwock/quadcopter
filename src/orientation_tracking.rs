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

pub struct OrientationTracker<M: Imu> {
    pub orientation: [DegreeFixed32; 3],
    pub last_gyro_timestamp: u16,
    pub last_gyro_data_halved: [DegreeFixed32; 3],
    pub imuctl: ImuController<M>,
}

//const DEGREE_SCALING_FACTOR: i32 = 64000;
const DEGREE_SCALING_FACTOR: i32 = 640;
const HALVED_DEGREE_SCALING_FACTOR: i32 = DEGREE_SCALING_FACTOR / 2;
pub fn reading_to_dps_32(reading: i16) -> DegreeFixed32 {
    DegreeFixed32::from_bits(reading as i32) * DEGREE_SCALING_FACTOR
}

// Why this?  Saves us a separate muliply or division later on
pub fn halved_reading_to_dps_32(reading: i16) -> DegreeFixed32 {
    DegreeFixed32::from_bits(reading as i32) * HALVED_DEGREE_SCALING_FACTOR
}

const TIME_SCALE_FACTOR: i32 = 1_099_512;
pub fn timestamp_diff_to_seconds(timestamp: u16) -> DegreeFixed32 {
    DegreeFixed32::from_bits(timestamp as i32) * DegreeFixed32::from_bits(TIME_SCALE_FACTOR)
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
            last_gyro_timestamp: 0,
            last_gyro_data_halved: [DegreeFixed32::from_bits(0); 3],
            imuctl,
        }
    }

    pub fn get_orientation(&self) -> [DegreeFixed32; 3] {
        self.orientation
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
            let _accel_data = msg.accel_data;
            let gyro_data = msg.gyro_data;
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
                self.last_gyro_data_halved[i] = new_halved_reading;
            }
            self.last_gyro_timestamp = timestamp;
        }
    }
}
