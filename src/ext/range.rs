use core::ops::{Range, Sub};

// is_empty exists for Range
#[allow(clippy::len_without_is_empty)]
pub trait RangeExt<T> {
    fn len(&self) -> Option<T>
    where
        T: Ord + Sub<Output = T> + Copy;

    fn is_valid(&self) -> bool
    where
        T: Ord;
}

impl<T> RangeExt<T> for Range<T> {
    fn len(&self) -> Option<T>
    where
        T: Ord + Sub<Output = T> + Copy,
    {
        if self.is_valid() {
            Some(self.end - self.start)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool
    where
        T: Ord,
    {
        self.start <= self.end
    }
}
