// Bad implementations of fixed point vector math operations lol.
use fixed::{FixedI16, FixedI32};
use esp_println::println;
use core::ops::{Add, Sub, Mul, Div, Shl, Shr};
use fixed::types::extra::{U8, U15, U11, U10, U5, U16, U20};
use crate::debug_println;
pub type UnityFixed16 = FixedI16<U15>;
pub type RadianFixed16 = FixedI16<U11>;
// Focus on the range of -180 to +180 degrees with some subdegree precision
pub type DegreeFixed16 = FixedI16<U5>;
pub type DegreeFixed32 = FixedI32<U20>;
use typenum::UTerm;
use az::{Cast, SaturatingCast};
use fixed_trigonometry::{
    atan::atan2,
    wrap_phase,
};

// For 2000dps, FixedI32<U10>: const DPS_BITS: i32 = 62;
// For 2000 dps, FixedI32<U15>
//const DPS_BITS: i32 = 64000;
// 1/16.4
//const DEGREES_PER_LSB: DegreeFixed32 = DegreeFixed32::from_bits(DPS_BITS);
//const DEGREES_PER_LSB_OVER_2: DegreeFixed32 = DegreeFixed32::from_bits(DPS_BITS / 2);


#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct MotionData {
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub gyr_x: i16,
    pub gyr_y: i16,
    pub gyr_z: i16,
}

impl MotionData {
    pub fn gyro_vec(&self) -> [i16; 3] {
        [self.gyr_x, self.gyr_y, self.gyr_z]
    }

    pub fn acc_vec(&self) -> [i16; 3] {
        [self.acc_x, self.acc_y, self.acc_z]
    }
    pub(crate) fn zero() -> Self {
        Self {
            acc_x: 0,
            acc_y: 0,
            acc_z: 0,
            gyr_x: 0,
            gyr_y: 0,
            gyr_z: 0,
        }
    }

    pub(crate) fn show(&self) {
        debug_println!("Acceleration: {{ x: {}, y: {}, z: {} }}, Gyro: {{ x: {}, y: {}, z: {} }}", self.acc_x, self.acc_y, self.acc_z, self.gyr_x, self.gyr_y, self.gyr_z);
    }

    pub(crate) fn into_vector(self) -> [i16; 6] {
        [self.acc_x, self.acc_y, self.acc_z, self.gyr_x, self.gyr_y, self.gyr_z]
    }

    pub fn from_vector(v: [i16; 6]) -> Self {
        Self {
            acc_x: v[0],
            acc_y: v[1],
            acc_z: v[2],
            gyr_x: v[3],
            gyr_y: v[4],
            gyr_z: v[5],
        }
    }

    pub fn acc_magnitude(self) -> i16 {
        FixedMotionData::from(self).acc_magnitude().to_bits()
    }
}


#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct FixedMotionData {
    pub acc_x: UnityFixed16,
    pub acc_y: UnityFixed16,
    pub acc_z: UnityFixed16,
    pub gyr_x: UnityFixed16,
    pub gyr_y: UnityFixed16,
    pub gyr_z: UnityFixed16,
}

fn compute_magnitude_15<const N: usize>(v: [FixedI16<U15>; N]) -> FixedI16<U15> {
    let up_v = v.map(|x| Cast::<FixedI32<U15>>::cast(x));
    up_v.iter().fold(FixedI32::<U15>::from_num(0), |acc, x| acc + x * x).sqrt()
        .saturating_cast()
}

fn normalize_vector_15<const N: usize>(v: [FixedI16<U15>; N]) -> [FixedI16<U15>; N] {
    debug_println!("Normalizing");
    let up_v = v.map(|x| Cast::<FixedI32<U15>>::cast(x));
    let max = up_v.iter().map(|x| x.abs()).max().unwrap();
    // Prevent div by zero
    if max == 0 {
        return [FixedI16::<U15>::from_num(0); N];
    }
    debug_println!("max recip");
    let max_recip = max.recip();
    let scaled = up_v.map(|x| x * max_recip);
    let square_sum = scaled.iter().map(|&x| x * x).sum::<FixedI32<U15>>();
    // Prevent div by zero again- this wouldn't be possible if we were using real numbers
    // but these are only represenations.
    if square_sum == 0 {
        return [FixedI16::<U15>::from_num(0); N];
    }
    debug_println!("inv_sqsum");
    let inv_sqsum = square_sum.sqrt().recip();
    scaled.map(|x| (x * inv_sqsum).saturating_cast())
}

fn compute_magnitude_12<const N: usize>(v: [FixedI16<U11>; N]) -> FixedI16<U11> {
    let up_v = v.map(|x| Cast::<FixedI32<U11>>::cast(x));
    up_v.iter().fold(FixedI32::<U11>::from_num(0), |acc, x| acc + x * x).sqrt()
        .saturating_cast()
}

fn normalize_vector_12<const N: usize>(v: [FixedI16<U11>; N]) -> [FixedI16<U11>; N] {
    debug_println!("Normalizing");
    let up_v = v.map(|x| Cast::<FixedI32<U11>>::cast(x));
    let max = up_v.iter().map(|x| x.abs()).max().unwrap();
    // Prevent div by zero
    if max == 0 {
        return [FixedI16::<U11>::from_num(0); N];
    }
    debug_println!("max recip");
    let max_recip = max.recip();
    let scaled = up_v.map(|x| x * max_recip);
    let square_sum = scaled.iter().map(|&x| x * x).sum::<FixedI32<U11>>();
    // Prevent div by zero again- this wouldn't be possible if we were using real numbers
    // but these are only represenations.
    if square_sum == 0 {
        return [FixedI16::<U11>::from_num(0); N];
    }
    debug_println!("inv_sqsum");
    let inv_sqsum = square_sum.sqrt().recip();
    scaled.map(|x| (x * inv_sqsum).saturating_cast())
}

impl FixedMotionData {
    pub fn acc_magnitude(self) -> FixedI16<U15> {
        compute_magnitude_15(self.into_acc_vector())
    }

    pub fn normalized_acc(self) -> [FixedI16<U15>; 3] {
        let vec = [self.acc_x, self.acc_y, self.acc_z];
        let normalized = normalize_vector_15(vec);
        if normalized.iter().all(|&x| x == UnityFixed16::from_bits(0)) {
            [UnityFixed16::from_num(0), UnityFixed16::MAX, UnityFixed16::from_num(0)]
        } else {
            normalized
        }
    }

    pub fn into_acc_vector(self) -> [FixedI16<U15>; 3] {
        [self.acc_x, self.acc_y, self.acc_z]
    }

    pub fn into_vector(self) -> [FixedI16<U15>; 6] {
        [self.acc_x, self.acc_y, self.acc_z, self.gyr_x, self.gyr_y, self.gyr_z]
    }

    pub fn from_vector(v: [FixedI16<U15>; 6]) -> Self {
        Self {
            acc_x: v[0],
            acc_y: v[1],
            acc_z: v[2],
            gyr_x: v[3],
            gyr_y: v[4],
            gyr_z: v[5],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct TiltData {
    pub accel_xz_tilt: RadianFixed16,
    pub accel_yz_tilt: RadianFixed16,
    pub accel_magnitude: RadianFixed16,
    pub gyro_x_ddt: RadianFixed16,
    pub gyro_y_ddt: RadianFixed16,
    pub gyro_z_ddt: RadianFixed16,
}

impl TiltData {
    pub fn tilt_vector(&self) -> [RadianFixed16; 2] {
        [self.accel_xz_tilt, self.accel_yz_tilt]
    }
}

pub fn to_unity(x: RadianFixed16) -> UnityFixed16 {
    UnityFixed16::from_bits(x.to_bits())
}

impl From<MotionData> for TiltData {
    fn from(other: MotionData) -> Self {
        // Calculate acceleration magnitude, x_tilt, and y_tilt
        let v = [FixedI32::<U11>::from_bits(other.acc_x as i32), FixedI32::<U11>::from_bits(other.acc_y as i32), FixedI32::<U11>::from_bits(other.acc_z as i32)];
        println!("Acc v {:?}", v);
        let v_sq = v.map(|x| x * x);
        println!("v^2 {:?}", v_sq);
        let acc_magnitude = v_sq.iter().fold(FixedI32::<U11>::from_num(0), |acc, &x| acc + x).sqrt();
        println!("Acc_magnitude {acc_magnitude}");
        let xz_orthogonal_mag = [v_sq[0], v_sq[2]].iter().fold(FixedI32::<U11>::from_num(0), |acc, &x| acc + x).sqrt();
        let yz_orthogonal_mag = [v_sq[1], v_sq[2]].iter().fold(FixedI32::<U11>::from_num(0), |acc, &x| acc + x).sqrt();
        let xz_tilt = atan2(v[0], yz_orthogonal_mag);
        let yz_tilt = atan2(v[1], xz_orthogonal_mag);
        Self {
            accel_xz_tilt: xz_tilt.saturating_cast(),
            accel_yz_tilt: yz_tilt.saturating_cast(),
            accel_magnitude: acc_magnitude.saturating_cast(),
            gyro_x_ddt: FixedI16::<U11>::from_bits(other.gyr_x),
            gyro_y_ddt: FixedI16::<U11>::from_bits(other.gyr_y),
            gyro_z_ddt: FixedI16::<U11>::from_bits(other.gyr_z),
        }
    }
}

impl From<MotionData> for FixedMotionData {
    fn from(other: MotionData) -> Self {
        Self {
            acc_x: UnityFixed16::from_bits(other.acc_x),
            acc_y: UnityFixed16::from_bits(other.acc_y),
            acc_z: UnityFixed16::from_bits(other.acc_z),
            gyr_x: UnityFixed16::from_bits(other.gyr_x),
            gyr_y: UnityFixed16::from_bits(other.gyr_y),
            gyr_z: UnityFixed16::from_bits(other.gyr_z),
        }
    }
}

impl Add for FixedMotionData {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            acc_x: self.acc_x.saturating_add(rhs.acc_x),
            acc_y: self.acc_y.saturating_add(rhs.acc_y),
            acc_z: self.acc_z.saturating_add(rhs.acc_z),
            gyr_x: self.gyr_x.saturating_add(rhs.gyr_x),
            gyr_y: self.gyr_y.saturating_add(rhs.gyr_y),
            gyr_z: self.gyr_z.saturating_add(rhs.gyr_z),
        }
    }
}

impl Sub for FixedMotionData {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            acc_x: self.acc_x.saturating_sub(rhs.acc_x),
            acc_y: self.acc_y.saturating_sub(rhs.acc_y),
            acc_z: self.acc_z.saturating_sub(rhs.acc_z),
            gyr_x: self.gyr_x.saturating_sub(rhs.gyr_x),
            gyr_y: self.gyr_y.saturating_sub(rhs.gyr_y),
            gyr_z: self.gyr_z.saturating_sub(rhs.gyr_z),
        }
    }
}

impl Mul for FixedMotionData {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            acc_x: self.acc_x.saturating_mul(rhs.acc_x),
            acc_y: self.acc_y.saturating_mul(rhs.acc_y),
            acc_z: self.acc_z.saturating_mul(rhs.acc_z),
            gyr_x: self.gyr_x.saturating_mul(rhs.gyr_x),
            gyr_y: self.gyr_y.saturating_mul(rhs.gyr_y),
            gyr_z: self.gyr_z.saturating_mul(rhs.gyr_z),
        }
    }
}

impl Shl<u16> for FixedMotionData {
    type Output = Self;
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            acc_x: self.acc_x << rhs,
            acc_y: self.acc_y << rhs,
            acc_z: self.acc_z << rhs,
            gyr_x: self.gyr_x << rhs,
            gyr_y: self.gyr_y << rhs,
            gyr_z: self.gyr_z << rhs,
        }
    }
}

impl Shr<u16> for FixedMotionData {
    type Output = Self;
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            acc_x: self.acc_x >> rhs,
            acc_y: self.acc_y >> rhs,
            acc_z: self.acc_z >> rhs,
            gyr_x: self.gyr_x >> rhs,
            gyr_y: self.gyr_y >> rhs,
            gyr_z: self.gyr_z >> rhs,
        }
    }
}

impl Add for MotionData {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            acc_x: self.acc_x.saturating_add(rhs.acc_x),
            acc_y: self.acc_y.saturating_add(rhs.acc_y),
            acc_z: self.acc_z.saturating_add(rhs.acc_z),
            gyr_x: self.gyr_x.saturating_add(rhs.gyr_x),
            gyr_y: self.gyr_y.saturating_add(rhs.gyr_y),
            gyr_z: self.gyr_z.saturating_add(rhs.gyr_z),
        }
    }
}

impl Sub for MotionData {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            acc_x: self.acc_x.saturating_sub(rhs.acc_x),
            acc_y: self.acc_y.saturating_sub(rhs.acc_y),
            acc_z: self.acc_z.saturating_sub(rhs.acc_z),
            gyr_x: self.gyr_x.saturating_sub(rhs.gyr_x),
            gyr_y: self.gyr_y.saturating_sub(rhs.gyr_y),
            gyr_z: self.gyr_z.saturating_sub(rhs.gyr_z),
        }
    }
}

impl Mul for MotionData {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            acc_x: self.acc_x.saturating_mul(rhs.acc_x),
            acc_y: self.acc_y.saturating_mul(rhs.acc_y),
            acc_z: self.acc_z.saturating_mul(rhs.acc_z),
            gyr_x: self.gyr_x.saturating_mul(rhs.gyr_x),
            gyr_y: self.gyr_y.saturating_mul(rhs.gyr_y),
            gyr_z: self.gyr_z.saturating_mul(rhs.gyr_z),
        }
    }
}

impl Shl<u16> for MotionData {
    type Output = Self;
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            acc_x: self.acc_x << rhs,
            acc_y: self.acc_y << rhs,
            acc_z: self.acc_z << rhs,
            gyr_x: self.gyr_x << rhs,
            gyr_y: self.gyr_y << rhs,
            gyr_z: self.gyr_z << rhs,
        }
    }
}

impl Shr<u16> for MotionData {
    type Output = Self;
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            acc_x: self.acc_x >> rhs,
            acc_y: self.acc_y >> rhs,
            acc_z: self.acc_z >> rhs,
            gyr_x: self.gyr_x >> rhs,
            gyr_y: self.gyr_y >> rhs,
            gyr_z: self.gyr_z >> rhs,
        }
    }
}
