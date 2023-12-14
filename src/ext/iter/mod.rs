pub mod chunks;
pub mod parse;

use self::{
    chunks::{Check, CheckType, ChunksExact, DebugCheck, NoCheck},
    parse::{parse_int, ParseIntError, ParseIntTo},
};
use core::{marker::PhantomData, str::FromStr};

pub trait IterExt: Iterator + Sized {
    fn chunks_const_generic<const N: usize, C: CheckType>(self) -> ChunksExact<Self, N, C> {
        ChunksExact(self, PhantomData)
    }

    fn chunks_const<const N: usize>(self) -> ChunksExact<Self, N, NoCheck> {
        Self::chunks_const_generic(self)
    }

    fn chunks_const_debug_check<const N: usize>(self) -> ChunksExact<Self, N, DebugCheck> {
        Self::chunks_const_generic(self)
    }

    fn chunks_const_check<const N: usize>(self) -> ChunksExact<Self, N, Check> {
        Self::chunks_const_generic(self)
    }

    fn parse_all<T: FromStr>(self) -> Result<Vec<T>, T::Err>
    where
        Self::Item: AsRef<str>,
    {
        self.map(|a| a.as_ref().parse()).collect()
    }

    fn parse_int<N: ParseIntTo>(self) -> Result<N, ParseIntError>
    where
        Self: Iterator<Item = u8>,
    {
        parse_int(self)
    }
}

impl<T: Iterator> IterExt for T {}
