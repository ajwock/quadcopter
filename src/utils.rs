use core::{
    ops::{Add, Sub},
    cmp::Ord,
};

pub fn asymmetrical_rate_limit<T: Add<Output=T> + Sub<Output=T> + Ord + Copy>(current: T, target: T, up_limit: T, down_limit: T) -> T {
    if current > target {
        let diff = current - target;
        let new_delta = core::cmp::min(diff, down_limit);
        current - new_delta
    } else {
        let diff = target - current;
        let new_delta = core::cmp::min(diff, up_limit);
        current + new_delta
    }
}

pub fn rate_limit<T: Add<Output=T> + Sub<Output=T> + Ord + Copy>(current: T, target: T, limit: T) -> T {
    asymmetrical_rate_limit(current, target, limit, limit)
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if false { println!($($arg)*) });
}
