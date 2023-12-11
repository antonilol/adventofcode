use core::fmt::Display;
use num_traits::{CheckedAdd, CheckedMul, CheckedSub, Zero};

pub(super) fn parse_int<I: Iterator<Item = u8>, N: ParseIntTo>(
    mut bytes: I,
) -> Result<N, ParseIntError> {
    let mut byte = bytes.next().ok_or(ParseIntError {
        kind: ParseIntErrorKind::Empty,
        error_pos: 0,
    })?;

    let mut i = 1usize;
    let neg = !N::UNSIGNED && byte == b'-';
    if N::UNSIGNED && byte == b'-' {
        return Err(ParseIntError {
            kind: ParseIntErrorKind::UnexpectedSign,
            error_pos: 0,
        });
    }
    if neg {
        byte = bytes.next().ok_or(ParseIntError {
            kind: ParseIntErrorKind::EmptyNegative,
            error_pos: 1,
        })?;
        i += 1;
    }

    let mut n = N::zero();

    loop {
        if byte.is_ascii_digit() {
            n = n
                .checked_mul(&N::TEN)
                .and_then(|n| {
                    if neg {
                        n.checked_sub(&N::from_ascii_digit(byte))
                    } else {
                        n.checked_add(&N::from_ascii_digit(byte))
                    }
                })
                .ok_or(ParseIntError {
                    kind: ParseIntErrorKind::Overflow,
                    error_pos: i,
                })?;
            byte = match bytes.next() {
                Some(byte) => byte,
                None => return Ok(n),
            };
            i += 1;
        } else {
            return Err(ParseIntError {
                kind: ParseIntErrorKind::InvalidByte(byte),
                error_pos: i,
            });
        }
    }
}

pub trait ParseIntTo: CheckedAdd + CheckedSub + CheckedMul + Zero {
    /// true if representations can never have a minus sign
    const UNSIGNED: bool;
    /// 10
    const TEN: Self;

    /// Convert from an ascii digit to the integer type. The character is guaranteed to be an ascii
    /// digit.
    fn from_ascii_digit(byte: u8) -> Self;
}

macro_rules! parse_int_to_impl {
    ($($t:ty)*) => {
        $(
            impl ParseIntTo for $t {
                const UNSIGNED: bool = Self::MIN == 0;
                const TEN: Self = 10;

                fn from_ascii_digit(byte: u8) -> Self {
                    (byte - b'0') as Self
                }
            }
        )*
    };
}

parse_int_to_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

#[derive(Debug, PartialEq)]
pub struct ParseIntError {
    pub kind: ParseIntErrorKind,
    pub error_pos: usize,
}

impl Display for ParseIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, valid up to index {}", self.kind, self.error_pos)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseIntErrorKind {
    Empty,
    EmptyNegative,
    UnexpectedSign,
    InvalidByte(u8),
    Overflow,
}

impl Display for ParseIntErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "iterator yielded no bytes"),
            Self::EmptyNegative => write!(f, "iterator yielded only a minus sign"),
            Self::UnexpectedSign => {
                write!(f, "unexpected minus sign when parsing unsigned integer")
            }
            Self::InvalidByte(byte) => {
                write!(
                    f,
                    "iterator yielded invalid byte '{}' ({byte:#04x})",
                    match *byte {
                        byte @ 32..=126 => byte as char,
                        _ => char::REPLACEMENT_CHARACTER,
                    }
                )
            }
            Self::Overflow => write!(
                f,
                "integer value overflowed/underflowed numeric bounds of the integer type"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_int, ParseIntError, ParseIntErrorKind::*};

    #[test]
    fn test_parse_int() {
        type TestCase<'a> = (
            &'a str,
            Result<u8, ParseIntError>,
            Result<i8, ParseIntError>,
            Result<u64, ParseIntError>,
            Result<i64, ParseIntError>,
        );
        // generated but reviewed
        const TEST_CASES: &[TestCase] = &[
            ("0", Ok(0), Ok(0), Ok(0), Ok(0)),
            ("1", Ok(1), Ok(1), Ok(1), Ok(1)),
            (
                "-1",
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(-1),
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(-1),
            ),
            ("127", Ok(127), Ok(127), Ok(127), Ok(127)),
            (
                "255",
                Ok(255),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Ok(255),
                Ok(255),
            ),
            (
                "-128",
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(-128),
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(-128),
            ),
            (
                "256",
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Ok(256),
                Ok(256),
            ),
            (
                "-129",
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(-129),
            ),
            (
                "999999",
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Ok(999999),
                Ok(999999),
            ),
            (
                "-999999",
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(-999999),
            ),
            (
                "9223372036854775807",
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Ok(9223372036854775807),
                Ok(9223372036854775807),
            ),
            (
                "18446744073709551615",
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Ok(18446744073709551615),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 20,
                }),
            ),
            (
                "-9223372036854775808",
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(-9223372036854775808),
            ),
            (
                "18446744073709551616",
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 3,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 20,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 20,
                }),
            ),
            (
                "-9223372036854775809",
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: Overflow,
                    error_pos: 20,
                }),
            ),
            (
                "",
                Err(ParseIntError {
                    kind: Empty,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: Empty,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: Empty,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: Empty,
                    error_pos: 0,
                }),
            ),
            ("00", Ok(0), Ok(0), Ok(0), Ok(0)),
            (
                "-0",
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(0),
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Ok(0),
            ),
            (
                "-",
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: EmptyNegative,
                    error_pos: 1,
                }),
                Err(ParseIntError {
                    kind: UnexpectedSign,
                    error_pos: 0,
                }),
                Err(ParseIntError {
                    kind: EmptyNegative,
                    error_pos: 1,
                }),
            ),
            (
                "abc",
                Err(ParseIntError {
                    kind: InvalidByte(97),
                    error_pos: 1,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(97),
                    error_pos: 1,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(97),
                    error_pos: 1,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(97),
                    error_pos: 1,
                }),
            ),
            (
                "123abc",
                Err(ParseIntError {
                    kind: InvalidByte(97),
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(97),
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(97),
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(97),
                    error_pos: 4,
                }),
            ),
            (
                "1.0",
                Err(ParseIntError {
                    kind: InvalidByte(46),
                    error_pos: 2,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(46),
                    error_pos: 2,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(46),
                    error_pos: 2,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(46),
                    error_pos: 2,
                }),
            ),
            (
                "123-",
                Err(ParseIntError {
                    kind: InvalidByte(45),
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(45),
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(45),
                    error_pos: 4,
                }),
                Err(ParseIntError {
                    kind: InvalidByte(45),
                    error_pos: 4,
                }),
            ),
        ];

        for c in TEST_CASES {
            assert_eq!(
                c,
                &(
                    c.0,
                    parse_int(c.0.bytes()),
                    parse_int(c.0.bytes()),
                    parse_int(c.0.bytes()),
                    parse_int(c.0.bytes()),
                )
            );
        }
    }
}
