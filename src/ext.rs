use core::{cell::Cell, ops::Add};

pub trait CellExt<T: Copy + Add<Output = T>> {
    fn add(&self, value: T);

    fn inc(&self)
    where
        T: PrimitiveInt;
}

impl<T: Copy + Add<Output = T>> CellExt<T> for Cell<T> {
    fn add(&self, value: T) {
        self.set(self.get() + value);
    }

    fn inc(&self)
    where
        T: PrimitiveInt,
    {
        self.add(T::ONE);
    }
}

pub trait PrimitiveInt {
    const ONE: Self;
}

macro_rules! int_impl {
    ($($t:ty)*) => {
        $(
            impl PrimitiveInt for $t {
                const ONE: Self = 1;
            }
        )*
    };
}

int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
