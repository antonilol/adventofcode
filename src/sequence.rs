use core::mem::replace;

pub struct Sequence<T, F: FnMut(&T) -> T> {
    curr: T,
    f: F,
}

impl<T, F: FnMut(&T) -> T> Sequence<T, F> {
    pub fn new(start: T, next: F) -> Self {
        Self {
            curr: start,
            f: next,
        }
    }
}

impl<T, F: FnMut(&T) -> T> Iterator for Sequence<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let new = (self.f)(&self.curr);

        Some(replace(&mut self.curr, new))
    }
}

#[cfg(test)]
mod tests {
    use super::Sequence;

    #[test]
    fn test_linear() {
        let seq = Sequence::new(0, |n| n + 1).take(8).collect::<Vec<_>>();
        assert_eq!(seq, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_exponential() {
        let seq = Sequence::new(1, |n| n + n).take(8).collect::<Vec<_>>();
        assert_eq!(seq, [1, 2, 4, 8, 16, 32, 64, 128]);
    }
}
