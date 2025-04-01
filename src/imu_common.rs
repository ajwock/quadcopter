use crate::motion_data::MotionData;
use smallvec::SmallVec;
use esp_println::println;

pub trait Imu {
    async fn read_motion_data_raw(&mut self) -> MotionData;
}

// The IMU calibrator must be run some opaque number of ticks until
// it yields a calibratoed Imu Controller.
pub struct ImuCalibrator<M: Imu> {
    imu_holder: Option<M>,
    calibration_data: SmallVec<[MotionData; 16]>,
}

impl<M: Imu> ImuCalibrator<M> {
    pub fn new(imu: M) -> Self {
        Self {
            imu_holder: Some(imu),
            calibration_data: SmallVec::new(),
        }
    }

    pub async fn calibration_tick(&mut self) -> Option<ImuController<M>> {
        let Some(imu_ref) = self.imu_holder.as_mut() else {
            panic!("Calibrator has yielded its imu");
        };
        self.calibration_data.push(imu_ref.read_motion_data_raw().await);
        if self.calibration_data.len() != self.calibration_data.capacity() {
            return None
        }

        let sum_vector = self.calibration_data.iter().map(|x| x.into_vector().map(|y| y as i32))
            .fold([0; 6], |acc, v| core::array::from_fn(|i| acc[i] + v[i]));
        let avg_offsets = sum_vector.map(|x| (x >> 4) as i16);
        let raw_offsets = MotionData::from_vector(avg_offsets);
        let magnitude = raw_offsets.acc_magnitude();
        let mut gravity_vector = MotionData::zero();
        gravity_vector.acc_z = magnitude;
        let calib_offsets = gravity_vector - raw_offsets;
        let calib_acc_magnitude = calib_offsets.acc_magnitude();
        if calib_acc_magnitude > 1000 {
            panic!("Calibration failed- offset magnitude {} > 1000.  Ensure the device is on a level surface", calib_acc_magnitude);
        }
        println!("Note: Calibration offset magnitude: {}", calib_acc_magnitude);
        Some(
            ImuController::new(self.imu_holder.take().unwrap())
                .with_calibration(calib_offsets)
                .with_gravmag(magnitude))
    }
}

pub struct ImuController<M: Imu> {
    imu: M,
    calibration_offsets: MotionData,
    gravity_magnitude: i16,
}

impl<M: Imu> ImuController<M> {
    pub fn new(imu: M) -> Self {
        Self {
            imu,
            calibration_offsets: MotionData::zero(),
            gravity_magnitude: i16::MAX / 4,
        }
    }

    pub fn with_calibration(self, calibration_offsets: MotionData) -> Self {
        Self {
            calibration_offsets,
            ..self
        }
    }

    pub fn with_gravmag(self, gravity_magnitude: i16) -> Self {
        Self {
            gravity_magnitude,
            ..self
        }
    }

    pub async fn read_motion_data(&mut self) -> MotionData {
        self.imu.read_motion_data_raw().await + self.calibration_offsets
    }

    pub fn gravity_mag(&self) -> i16 {
        self.gravity_magnitude
    }
}
