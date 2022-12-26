extern crate num;

use std::iter::{Sum, Product};
use num::{PrimInt, Unsigned};

#[inline]
pub fn parse_u8(b: &[u8]) -> u8 {
    b.iter().fold(0, |acc, x| acc * 10 + (x - b'0'))
}

#[inline]
pub fn parse<T>(b: &[u8]) -> T where
    T: PrimInt + Unsigned + Sum + Product
{
    b.iter().fold(T::zero(), |acc, x| acc * T::from(10).unwrap() + T::from(x - b'0').unwrap())
}
