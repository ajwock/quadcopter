use crate::motion_data::MotionData;
use smallvec::SmallVec;
use esp_println::println;
use embassy_futures;

#[derive(Copy, Clone, Debug)]
pub enum ImuErrorType {
    Unsupported, 
    MissingPacketInfo,
    CommunicationError,
    NotReady,
}

#[derive(Copy, Clone, Debug)]
pub struct ImuError {
    error_type: ImuErrorType,
}

impl ImuError {
    fn new(ty: ImuErrorType) -> Self {
        Self {
            error_type: ty,
        }
    }

    pub fn unsupported() -> Self {
        Self::new(ImuErrorType::Unsupported)
    }

    pub fn missing_info() -> Self {
        Self::new(ImuErrorType::MissingPacketInfo)
    }

    pub fn comms_error() -> Self {
        Self::new(ImuErrorType::CommunicationError)
    }

    pub fn not_ready() -> Self {
        Self::new(ImuErrorType::NotReady)
    }

    pub fn is_not_ready(&self) -> bool {
        match self.error_type {
            ImuErrorType::NotReady => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ImuMsg {
    pub accel_data: [i16; 3],
    pub gyro_data:  [i16; 3],
    pub timestamp:  u16,
}

impl ImuMsg {
    pub fn new(accel_data: [i16; 3], gyro_data: [i16; 3], timestamp: u16) -> Self {
        Self {
            accel_data,
            gyro_data,
            timestamp,
        }
    }

    pub fn as_motion_data(&self) -> MotionData {
        let acc = self.accel_data;
        let gyr = self.gyro_data;
        MotionData::from_vector([acc[0], acc[1], acc[2], gyr[0], gyr[1], gyr[2]])
    }

    pub fn with_calibration_data(self, calibration_offsets: MotionData) -> Self {
        let offset_data = self.as_motion_data() + calibration_offsets;
        Self {
            accel_data: offset_data.acc_vec(),
            gyro_data: offset_data.gyro_vec(),
            timestamp: self.timestamp,
        }
    }
}

pub trait Imu {
    // Read the present motion data
    async fn read_motion_data_raw(&mut self) -> MotionData;
    // Non-blockingly try to recieve a motion data msg
    async fn get_motion_data_msg(&mut self) -> Result<ImuMsg, ImuError> {
        Err(ImuError::unsupported())
    }

    async fn wait_for_motion_data_msg(&mut self) -> Result<ImuMsg, ImuError> {
        loop {
            break match self.get_motion_data_msg().await {
                Ok(m) => Ok(m),
                Err(e) if e.is_not_ready() => {
                    embassy_futures::yield_now().await;
                    continue
                }
                Err(e) => Err(e),
            }
        }
    }

    async fn flush_msgs(&mut self) {
        // Do nothing unless supported}
    }
}

// The IMU calibrator must be run some opaque number of ticks until
// it yields a calibratoed Imu Controller.
pub struct ImuCalibrator<M: Imu, const N: usize> {
    imu_holder: Option<M>,
    calibration_data: SmallVec<[MotionData; N]>,
}

impl<M: Imu, const N: usize> ImuCalibrator<M, N> {
    pub fn new(imu: M) -> Self {
        Self {
            imu_holder: Some(imu),
            calibration_data: SmallVec::new(),
        }
    }

    fn init_from_calibration_data(&mut self) -> ImuController<M> {
        let sum_vector = self.calibration_data.iter().map(|x| x.into_vector().map(|y| y as i32))
            .inspect(|y| println!("y cast: {:?}", y))
            .fold([0; 6], |acc, v| core::array::from_fn(|i| acc[i] + v[i]));
        println!("Sum_vector: {:?}", sum_vector);
        let avg_offsets = sum_vector.map(|x| (x / (N as i32)) as i16);
        let raw_offsets = MotionData::from_vector(avg_offsets);
        let magnitude = raw_offsets.acc_magnitude();
        let mut gravity_vector = MotionData::zero();
        gravity_vector.acc_z = magnitude;
        let calib_offsets = gravity_vector - raw_offsets;
        println!("Gravity vector: {:?}", gravity_vector);
        println!("raw_offsets: {:?}", raw_offsets);
        println!("calib_offsets: {:?}", calib_offsets);
        let calib_acc_magnitude = calib_offsets.acc_magnitude();
        if calib_acc_magnitude > 1000 {
            panic!("Calibration failed- offset magnitude {} > 1000.  Ensure the device is on a level surface", calib_acc_magnitude);
        }
        println!("Note: Calibration offset magnitude: {}", calib_acc_magnitude);
        println!("Calibration data: {:?}, calibration_offsets: {:?}", self.calibration_data, calib_offsets);
        ImuController::new(self.imu_holder.take().unwrap())
            .with_calibration(calib_offsets)
            .with_gravmag(magnitude)
    }

    pub async fn msg_calibration(&mut self) -> Result<ImuController<M>, ImuError> {
        loop {
            let imu_ref = self.imu_holder.as_mut().expect("Calibrator has yielded its imu");
            let imu_msg = match imu_ref.get_motion_data_msg().await {
                Ok(msg) => msg,
                Err(e) if e.is_not_ready() => {
                    embassy_futures::yield_now().await;
                    continue 
                },
                Err(e) => {
                    println!("Got error: {:?}", e);
                    continue
                }
            };
            self.calibration_data.push(imu_msg.as_motion_data());
            if self.calibration_data.len() == self.calibration_data.capacity() {
                return Ok(self.init_from_calibration_data())
            }
        }
    }

    pub async fn calibration_tick(&mut self) -> Option<ImuController<M>> {
        let imu_ref = self.imu_holder.as_mut().expect("Calibrator has yielded its imu");
        self.calibration_data.push(imu_ref.read_motion_data_raw().await);
        if self.calibration_data.len() != self.calibration_data.capacity() {
            return None
        } else {
            Some(self.init_from_calibration_data())
        }
    }
}

pub struct ImuController<M: Imu> {
    pub imu: M,
    pub calibration_offsets: MotionData,
    pub gravity_magnitude: i16,
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

    pub async fn get_motion_data_msg(&mut self) -> Result<ImuMsg, ImuError> {
        self.imu.get_motion_data_msg().await.map(|x| x.with_calibration_data(self.calibration_offsets))
    }

    pub fn gravity_mag(&self) -> i16 {
        self.gravity_magnitude
    }

    pub async fn flush_msgs(&mut self) {
        self.imu.flush_msgs().await
    }
}
