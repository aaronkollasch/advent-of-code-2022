use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
#[cfg(debug_assertions)]
use std::str;

#[inline]
fn parse(b: &[u8]) -> i32 {
    b.iter().fold(0_i32, |acc, x| acc * 10 + (x - b'0') as i32)
}

fn compare_lists(left: &[u8], right: &[u8]) -> Ordering {
    #[cfg(debug_assertions)]
    {
        println!("compare");
        println!("{}", str::from_utf8(&left).unwrap());
        println!("{}", str::from_utf8(&right).unwrap());
    }
    let (mut depth_l, mut depth_r) = (0, 0);
    let mut left = left
        .split(move |b| match *b {
            b',' => depth_l == 0,
            b'[' => {
                depth_l += 1;
                false
            }
            b']' => {
                depth_l -= 1;
                false
            }
            _ => false,
        })
        .filter(|v| v.len() > 0);
    let mut right = right
        .split(move |b| match *b {
            b',' => depth_r == 0,
            b'[' => {
                depth_r += 1;
                false
            }
            b']' => {
                depth_r -= 1;
                false
            }
            _ => false,
        })
        .filter(|v| v.len() > 0);
    loop {
        let (l, r) = (left.next(), right.next());
        match (l.is_none(), r.is_none()) {
            (true, true) => {
                return Equal;
            }
            (true, false) => {
                return Less;
            }
            (false, true) => {
                return Greater;
            }
            (false, false) => {}
        }
        let (l, r) = (l.unwrap(), r.unwrap());
        #[cfg(debug_assertions)]
        println!("-- {:?} {:?}", l, r);
        let result = match (l[0], r[0]) {
            (b'[', b'[') => compare_lists(&l[1..l.len() - 1], &r[1..r.len() - 1]),
            (b'[', _) => compare_lists(&l[1..l.len() - 1], r),
            (_, b'[') => compare_lists(l, &r[1..r.len() - 1]),
            _ => parse(l).cmp(&parse(r)),
        };
        #[cfg(debug_assertions)]
        println!(
            "{}",
            match result {
                Greater => "not ordered",
                Less => "ordered",
                Equal => "no decision",
            }
        );
        match result {
            Greater => return Greater,
            Less => return Less,
            Equal => {}
        };
    }
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    const FIRST: &[u8] = b"[[2]]";
    const SECOND: &[u8] = b"[[6]]";
    let result = s
        .split(|b| *b == b'\n')
        .filter(|l| l.len() > 0)
        .filter(|l| {
            compare_lists(
                &l[1..l.len() - 1],
                &SECOND[1..SECOND.len() - 1],
            ) != Greater
        })
        .chain([FIRST, SECOND].into_iter())
        .sorted_unstable_by(|left, right| {
            compare_lists(
                &left[1..left.len() - 1],
                &right[1..right.len() - 1],
            )
        })
        .collect::<Vec<&[u8]>>();
    #[cfg(debug_assertions)]
    {
        println!("{}", str::from_utf8(s).unwrap());
        println!("{}", result.iter().map(|l| str::from_utf8(l).unwrap()).join("\n"));
    }
    print!(
        "{} ",
        result.len() * (result.into_iter().rposition(|s| s == FIRST).unwrap() + 1)
    );
}
