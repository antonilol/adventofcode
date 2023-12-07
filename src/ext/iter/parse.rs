pub trait IntoByteIterator {
    type Iterator: Iterator<Item = u8>;

    fn into_byte_iter(self) -> Self::Iterator;
}

pub struct FlatByteIter<T: IntoByteIterator, I: Iterator<Item = T>> {
    iter: I,
    byte_iter: Option<T::Iterator>,
}

impl<T: IntoByteIterator, I: Iterator<Item = T>> Iterator for FlatByteIter<T, I> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match &mut self.byte_iter {
                Some(iter) => iter.next(),
                None => {
                    let mut byte_iter = self.iter.next()?.into_byte_iter();
                    let next = byte_iter.next();
                    self.byte_iter = Some(byte_iter);
                    next
                }
            };

            if let Some(byte) = next {
                return Some(byte);
            }
        }
    }
}

impl<T: IntoByteIterator, I: Iterator<Item = T>> IntoByteIterator for I {
    type Iterator = FlatByteIter<T, Self>;

    fn into_byte_iter(self) -> Self::Iterator {
        FlatByteIter {
            iter: self,
            byte_iter: None,
        }
    }
}

pub(super) fn parse_int<I: IntoByteIterator, N: ParseIntTo>(iter: I) -> Result<N, ParseIntError> {
    let mut bytes = iter.into_byte_iter();
    let first_byte = bytes.next().ok_or(ParseIntError {
        kind: ParseIntErrorKind::NotANumber,
        valid_up_to: 0,
    })?;

    
    let mut max;
    if !N::UNSIGNED && first_byte == b'-' {
        max = 
    }

    if first_byte.is_ascii_digit() {}

    Ok( )
}

pub trait ParseIntTo {
    const UNSIGNED: bool;
    const MIN: Self;
    const MAX: Self;
}

macro_rules! parse_int_to_impl {
    ($($t:ty)*) => {
        $(
            impl ParseIntTo for $t {
                const UNSIGNED: bool = Self::MIN == 0;
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;
            }
        )*
    };
}

parse_int_to_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

pub struct ParseIntError {
    pub kind: ParseIntErrorKind,
    pub valid_up_to: usize,
}

pub enum ParseIntErrorKind {
    NotANumber,
    InvalidByte(u8),
}
