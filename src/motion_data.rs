use fixed::{FixedI16, FixedI32};
use esp_println::println;
use core::ops::{Add, Sub, Mul, Div, Shl, Shr};
use fixed::types::extra::{U8, U15, U13};
use az::{Cast, CheckedCast};

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
    pub(crate) fn show(&self) {
        println!("Acceleration: {{ x: {}, y: {}, z: {} }}, Gyro: {{ x: {}, y: {}, z: {} }}", self.acc_x, self.acc_y, self.acc_z, self.gyr_x, self.gyr_y, self.gyr_z);
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
/*
fn normalize_vector<const N: usize>(v: [FixedI16<U15>; N]) -> [UnityFixed16; N] {
    let up_v = v.map(|x| Cast::<FixedI16<U13>>::cast(x));
    let max = up_v.iter().map(|x| x.abs()).max().unwrap();
    // Prevent div by zero
    if max == 0 {
        return [FixedI16::<U15>::from_num(0); N];
    }
    let max_recip = max.recip();
    let scaled = up_v.map(|x| x * max_recip);
    let square_sum = scaled.iter().map(|&x| x * x).sum::<FixedI16<U13>>();
    // Prevent div by zero again- this wouldn't be possible if we were using real numbers
    // but these are only represenations.
    if square_sum == 0 {
        return [FixedI16::<U15>::from_num(0); N];
    }
    let inv_sqsum = square_sum.sqrt().recip();
    scaled.map(|x| CheckedCast::<FixedI16<U15>>::checked_cast(x.saturating_mul(inv_sqsum)).unwrap_or(FixedI16::<U15>::from_num(0.9999)))
}*/

fn normalize_vector<const N: usize>(v: [FixedI16<U15>; N]) -> [UnityFixed16; N] {
    let up_v = v.map(|x| Cast::<FixedI32<U15>>::cast(x));
    let max = up_v.iter().map(|x| x.abs()).max().unwrap();
    // Prevent div by zero
    if max == 0 {
        return [FixedI16::<U15>::from_num(0); N];
    }
    let max_recip = max.recip();
    let scaled = up_v.map(|x| x * max_recip);
    let square_sum = scaled.iter().map(|&x| x * x).sum::<FixedI32<U15>>();
    // Prevent div by zero again- this wouldn't be possible if we were using real numbers
    // but these are only represenations.
    if square_sum == 0 {
        return [FixedI16::<U15>::from_num(0); N];
    }
    let inv_sqsum = square_sum.sqrt().recip();
    scaled.map(|x| Cast::<FixedI16<U15>>::cast(x * inv_sqsum))
}

impl FixedMotionData {
    pub fn normalized_acc(self) -> [FixedI16<U15>; 3] {
        let vec = [self.acc_x, self.acc_y, self.acc_z];
        let normalized = normalize_vector(vec);
        if normalized.iter().all(|&x| x == UnityFixed16::from_bits(0)) {
            [UnityFixed16::from_num(0), UnityFixed16::MIN, UnityFixed16::from_num(0)]
        } else {
            normalized
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
            acc_x: self.acc_x * rhs.acc_x,
            acc_y: self.acc_y * rhs.acc_y,
            acc_z: self.acc_z * rhs.acc_z,
            gyr_x: self.gyr_x * rhs.gyr_x,
            gyr_y: self.gyr_y * rhs.gyr_y,
            gyr_z: self.gyr_z * rhs.gyr_z,
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
