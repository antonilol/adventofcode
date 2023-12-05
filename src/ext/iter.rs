use core::{
    marker::PhantomData,
    mem::{transmute_copy, MaybeUninit},
};

pub trait IterExt: Iterator {
    fn chunks_const_generic<const N: usize, C: CheckType>(self) -> ChunksExact<Self, N, C>
    where
        Self: Sized;

    fn chunks_const<const N: usize>(self) -> ChunksExact<Self, N, NoCheck>
    where
        Self: Sized,
    {
        Self::chunks_const_generic(self)
    }

    fn chunks_const_debug_check<const N: usize>(self) -> ChunksExact<Self, N, DebugCheck>
    where
        Self: Sized,
    {
        Self::chunks_const_generic(self)
    }

    fn chunks_const_check<const N: usize>(self) -> ChunksExact<Self, N, Check>
    where
        Self: Sized,
    {
        Self::chunks_const_generic(self)
    }
}

impl<T: Iterator> IterExt for T {
    fn chunks_const_generic<const N: usize, C: CheckType>(self) -> ChunksExact<Self, N, C>
    where
        Self: Sized,
    {
        ChunksExact(self, PhantomData)
    }
}

pub trait CheckType {
    fn handle_invalid_state(n: usize, i: usize);
}

pub enum NoCheck {}

impl CheckType for NoCheck {
    fn handle_invalid_state(_n: usize, _i: usize) {}
}

pub enum DebugCheck {}

impl CheckType for DebugCheck {
    fn handle_invalid_state(n: usize, i: usize) {
        #[cfg(debug_assertions)]
        Check::handle_invalid_state(n, i);
    }
}

pub enum Check {}

impl CheckType for Check {
    fn handle_invalid_state(n: usize, i: usize) {
        panic!("chunks_const_check iterator expects an amount of elements divisible by {n}, got {n}*k+{i} instead.");
    }
}

pub struct ChunksExact<I: Iterator, const N: usize, C: CheckType>(I, PhantomData<C>);

impl<I: Iterator, const N: usize, C: CheckType> Iterator for ChunksExact<I, N, C> {
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut arr = uninit_array();

        for (i, e) in arr.iter_mut().enumerate() {
            if let Some(val) = self.0.next() {
                e.write(val);
            } else {
                if i > 0 {
                    C::handle_invalid_state(N, i);
                }
                return None;
            }
        }

        Some(unsafe { array_assume_init(arr) })
    }
}

// functions below are copied from the standard lib because they are unstable

fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}

unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY:
    // * The caller guarantees that all elements of the array are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    // And thus the conversion is safe
    unsafe {
        // intrinsics::assert_inhabited::<[T; N]>();
        // intrinsics::transmute_unchecked(array)
        transmute_copy(&array)
    }
}

#[cfg(test)]
mod tests {
    use super::IterExt;

    #[test]
    fn test_chunk_iter() {
        assert_eq!(
            (0..8)
                .chunks_const()
                .flat_map(|[a, b]| [b, a])
                .collect::<Vec<_>>(),
            [1, 0, 3, 2, 5, 4, 7, 6]
        );
        assert_eq!(
            (0..8)
                .chunks_const()
                .flat_map(|[a, b, c]| [c, b, a])
                .collect::<Vec<_>>(),
            [2, 1, 0, 5, 4, 3]
        );
    }
}
