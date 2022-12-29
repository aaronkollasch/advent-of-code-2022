extern crate num;

use num::{PrimInt, Signed, Unsigned};
use std::fmt;
use std::{
    fmt::{Binary, Display},
    iter::{Product, Sum},
    mem::size_of,
    ops::{BitAnd, BitOrAssign, Shl, Shr},
};

#[inline]
pub fn parse_u8(b: &[u8]) -> u8 {
    b.iter().fold(0, |acc, x| acc * 10 + (x - b'0'))
}

#[inline]
pub fn parse<T>(b: &[u8]) -> T
where
    T: PrimInt + Unsigned + Sum + Product,
{
    b.iter().fold(T::zero(), |acc, x| {
        acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
    })
}

#[inline]
pub fn parse_signed<T>(b: &[u8]) -> T
where
    T: PrimInt + Signed + Sum + Product,
{
    match b[0] {
        b'-' => {
            b[1..].iter().fold(T::zero(), |acc, x| {
                acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
            }) * T::from(-1).unwrap()
        }
        _ => b.iter().fold(T::zero(), |acc, x| {
            acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
        }),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitVec<T> {
    vec: T,
    size: usize,
}

impl<T> BitVec<T>
where
    T: PrimInt
        + Unsigned
        + BitOrAssign
        + Shr<Output = T>
        + Shl<Output = T>
        + BitAnd<Output = T>
        + Display
{
    pub fn new(size: usize) -> Self {
        if size > size_of::<T>() * 8 {
            panic!("too many bits for BitVec: {}", size);
        }
        Self {
            vec: T::zero(),
            size,
        }
    }

    #[inline]
    pub fn set_bit(&mut self, pos: T) {
        self.vec |= T::one() << pos;
    }

    #[inline]
    pub fn get_bit(&self, pos: T) -> T {
        (self.vec >> pos) & T::one()
    }

    #[inline]
    pub fn iter_unset(&self) -> impl Iterator<Item = T> + '_ {
        num::iter::range(T::zero(), T::from(self.size).unwrap())
            .filter(|i| self.get_bit(*i) == T::zero())
    }
}

impl<T> fmt::Display for BitVec<T>
where
    T: PrimInt + Binary,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "BitVec({})",
            format!("{:064b}", self.vec.reverse_bits())
                .split_at(self.size)
                .0
        )
    }
}
