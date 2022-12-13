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
            (true, true) => { return Equal; }
            (true, false) => { return Less; }
            (false, true) => { return Greater; }
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
                Less => "ordered",
                Greater => "not ordered",
                Equal => "no decision",
            }
        );
        match result {
            Less => return Less,
            Greater => return Greater,
            Equal => {}
        };
    }
}

pub fn main() {
    let s = include_str!("../input.txt");
    let result = s
        .split("\n\n")
        .filter(|g| g.len() > 0)
        .enumerate()
        .map(|(i, g)| (i, g.split_once("\n").unwrap()))
        .filter_map(|(i, (left, right))| {
            let ordering = compare_lists(
                left[1..left.len() - 1].as_bytes(),
                right[1..right.len() - 1].as_bytes(),
            );
            #[cfg(debug_assertions)]
            println!(
                "{}\n",
                match ordering {
                    Less => "CORRECT ORDER",
                    Greater => "NOT ORDERED",
                    Equal => "NO DECISION",
                }
            );
            match ordering {
                Less => Some(i + 1),
                _ => None,
            }
        })
        .sum::<usize>();
    print!("{} ", result);
}
