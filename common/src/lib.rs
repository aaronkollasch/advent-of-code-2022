extern crate num;

use std::iter::{Sum, Product};
use num::{PrimInt, Unsigned, Signed};

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

#[inline]
pub fn parse_signed<T>(b: &[u8]) -> T where
    T: PrimInt + Signed + Sum + Product
{
    match b[0] {
        b'-' => {
            b[1..].iter().fold(T::zero(), |acc, x| acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()) * T::from(-1).unwrap()
        }
        _ => {
            b.iter().fold(T::zero(), |acc, x| acc * T::from(10).unwrap() + T::from(x - b'0').unwrap())
        }
    }
}
