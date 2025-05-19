pub struct DelayBuf<T: Clone, const N: usize> {
    pub(crate) buf: [T; N],
    pub(crate) position: usize,
    default: T,
}

impl<T: Clone, const N: usize> DelayBuf<T, N> {
    pub fn new_with_default(default: T) -> Self {
        assert!(N != 0);
        Self {
            buf: core::array::from_fn(|_| default.clone()),
            position: 0,
            default,
        }
    }

    fn reset_with_default(&mut self, default: T) {
        self.buf = core::array::from_fn(|_| default.clone());
        self.default = default;
        self.position = 0;
    }

    pub fn reset(&mut self) {
        self.reset_with_default(self.default.clone());
        self.position = 0;
    }

    pub fn delay(&mut self, elt: T) -> T {
        let out = core::mem::replace(&mut self.buf[self.position], elt);
        self.position = if (self.position + 1) >= N {
            0
        } else {
            self.position + 1
        };
        out
    }

    pub fn iter<'a>(&'a self) -> DelayBufIter<'a, T, N> {
        DelayBufIter::new(self)
    }
}

// Iterator over each element in the delay line.  Always returns N elements.
pub struct DelayBufIter<'a, T: Clone, const N: usize> {
    delay_buf: &'a DelayBuf<T, N>,
    position: usize,
    walked: usize,
}

impl<'a, T: Clone, const N: usize> DelayBufIter<'a, T, N> {
    fn new(delay_buf: &'a DelayBuf<T, N>) -> Self {
        let position = delay_buf.position;
        Self {
            position,
            walked: 0,
            delay_buf,
        }
    }
}

impl<'a, T: Clone, const N: usize> Iterator for DelayBufIter<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.walked == N {
            return None
        }
        let ret = Some(&self.delay_buf.buf[self.position]);
        self.walked += 1;
        self.position = if (self.position + 1) >= N {
            0
        } else {
            self.position + 1
        };
        ret
    }
}
