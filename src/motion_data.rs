// Bad implementations of fixed point vector math operations lol.
use fixed::{FixedI16, FixedI32};
use esp_println::println;
use core::ops::{Add, Sub, Mul, Div, Shl, Shr};
use fixed::types::extra::{U8, U15, U13};
use az::{Cast, CheckedCast, SaturatingCast};
use crate::debug_println;
pub type UnityFixed16 = FixedI16<U15>;

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

fn compute_magnitude<const N: usize>(v: [FixedI16<U15>; N]) -> FixedI16<U15> {
    let up_v = v.map(|x| Cast::<FixedI32<U15>>::cast(x));
    up_v.iter().fold(FixedI32::<U15>::from_num(0), |acc, x| acc + x * x).sqrt()
        .saturating_cast()
}

fn normalize_vector<const N: usize>(v: [FixedI16<U15>; N]) -> [UnityFixed16; N] {
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

impl FixedMotionData {
    pub fn acc_magnitude(self) -> FixedI16<U15> {
        compute_magnitude(self.into_acc_vector())
    }

    pub fn normalized_acc(self) -> [FixedI16<U15>; 3] {
        let vec = [self.acc_x, self.acc_y, self.acc_z];
        let normalized = normalize_vector(vec);
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
