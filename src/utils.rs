use core::{
    ops::{Add, Sub},
    cmp::Ord,
};

pub fn rate_limit<T: Add<Output=T> + Sub<Output=T> + Ord + Copy>(current: T, target: T, limit: T) -> T {
    if current > target {
        let diff = current - target;
        let new_delta = core::cmp::min(diff, limit);
        current - new_delta
    } else {
        let diff = target - current;
        let new_delta = core::cmp::min(diff, limit);
        current + new_delta
    }
}
