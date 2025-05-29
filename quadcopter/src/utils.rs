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

pub struct DigitalPLL {
    divisor: u32,
    counter: u32,
}

impl DigitalPLL {
    pub fn new(divisor: u32) -> Self {
        Self {
            divisor,
            counter: 0,
        }
    }
    pub fn with_initial_count(self, counter: u32) -> Self {
        // This is intended to be helpful, though it isn't essential to prevent bad behavior
        assert!(counter <= self.divisor);
        Self {
            counter,
            ..self
        }
    }
    pub fn tick<F: FnOnce()>(&mut self, f: F) {
        if self.counter >= self.divisor {
            f();
            self.counter = 0;
        } else {
            self.counter += 1;
        }
    }
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if false { println!($($arg)*) });
}
