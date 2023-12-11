pub mod chunks;
pub mod parse;

use self::{
    chunks::{Check, CheckType, ChunksExact, DebugCheck, NoCheck},
    parse::{parse_int, ParseIntError, ParseIntTo},
};
use core::marker::PhantomData;

pub trait IterExt: Iterator {
    fn chunks_const_generic<const N: usize, C: CheckType>(self) -> ChunksExact<Self, N, C>
    where
        Self: Sized,
    {
        ChunksExact(self, PhantomData)
    }

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

    fn parse_int<N: ParseIntTo>(self) -> Result<N, ParseIntError>
    where
        Self: Iterator<Item = u8> + Sized,
    {
        parse_int(self)
    }
}

impl<T: Iterator> IterExt for T {}
