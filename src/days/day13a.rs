use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::str;

#[inline]
fn get_int(b: &[u8], idx: usize) -> (u8, usize) {
    match (b[idx], b[idx + 1]) {
        (b'1', b'0') => (10, idx + 2),
        (c, _) => (c - b'0', idx + 1),
    }
}

fn compare_lists(left: &[u8], right: &[u8], left_idx: usize, right_idx: usize) -> Ordering {
    #[cfg(debug_assertions)]
    {
        println!("compare");
        println!("{}", str::from_utf8(left).unwrap());
        println!("{}", str::from_utf8(right).unwrap());
    }
    let (mut left_idx, mut right_idx) = (left_idx, right_idx);
    let (mut left_depth, mut right_depth) = (0, 0);
    let (mut left_num, mut right_num);
    loop {
        match (left[left_idx], right[right_idx]) {
            (b']', b']') => {
                left_idx += 1;
                right_idx += 1;
            }
            (b',' | b']', _) if left_depth > 0 => {
                left_depth -= 1;
                if left_depth == 0 && left[left_idx] == b',' {
                    return Less;
                }
            }
            (_, b',' | b']') if right_depth > 0 => {
                right_depth -= 1;
                if right_depth == 0 && right[right_idx] == b',' {
                    return Greater;
                }
            }
            (_, b']') => return Greater,
            (b']', _) => return Less,
            (b'0'..=b'9', b'0'..=b'9') => {
                (left_num, left_idx) = get_int(left, left_idx);
                (right_num, right_idx) = get_int(right, right_idx);
                match left_num.cmp(&right_num) {
                    Equal => {
                        continue;
                    }
                    ord => return ord,
                }
            }
            (l, r) if l == r => {
                left_idx += 1;
                right_idx += 1;
            }
            (b'[', _) => {
                left_idx += 1;
                right_depth += 1;
            }
            (_, b'[') => {
                right_idx += 1;
                left_depth += 1;
            }
            (l, r) => {
                let left = str::from_utf8(left).unwrap();
                let right = str::from_utf8(right).unwrap();
                panic!(
                    "Found {l} ({left_idx}) and {r} ({right_idx}) for \"{left}\" and \"{right}\""
                )
            }
        }
    }
}

pub fn get_result() -> usize {
    let s = include_str!("../../inputs/day13.txt");
    let result = s
        .lines()
        .filter(|l| !l.is_empty())
        .tuples()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            let ordering = compare_lists(left.as_bytes(), right.as_bytes(), 0, 0);
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
    return result;
}

pub fn main() {
    print!("{} ", get_result());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        let result = get_result();
        assert_eq!(result, 5198);
    }
}
