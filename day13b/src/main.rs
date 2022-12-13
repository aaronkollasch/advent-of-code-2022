use itertools::Itertools;
use std::cmp::Ordering;
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
    let left = left[1..left.len()-1].split(|b| match *b {
        b',' => { depth_l == 0 }
        b'[' => { depth_l += 1; false }
        b']' => { depth_l -= 1; false }
        _ => { false }
    }).filter(|v| v.len() > 0).collect::<Vec<&[u8]>>();
    let right = right[1..right.len()-1].split(|b| match *b {
        b',' => { depth_r == 0 }
        b'[' => { depth_r += 1; false }
        b']' => { depth_r -= 1; false }
        _ => { false }
    }).filter(|v| v.len() > 0).collect::<Vec<&[u8]>>();
    let mut result = left.iter().zip(right.iter()).find_map(|(l, r)| {
            #[cfg(debug_assertions)]
            println!("-- {:?} {:?}", l, r);
            let r = match (l[0], r[0]) {
                (b'[', b'[') => { compare_lists(l, r) }
                (b'[', _) => { compare_lists(l, &[b"[", *r, b"]"].concat()) }
                (_, b'[') => { compare_lists(&[b"[", *l, b"]"].concat(), r) }
                _ => {
                    let (a, b) = (parse(l), parse(r));
                    b.cmp(&a)
                }
            };
            match r {
                Ordering::Equal => None,
                _ => Some(r),
            }
        });
    if result == None {
        result = if left.len() != right.len() { Some(right.len().cmp(&left.len())) }
        else { Some(Ordering::Equal) }
    }
    let result = result.unwrap();
    #[cfg(debug_assertions)]
    match result {
        Ordering::Greater => {
            println!("ordered");
        }
        Ordering::Less => {
            println!("not ordered");
        }
        Ordering::Equal => {
            println!("no decision");
        }
    }
    result
}

pub fn main() {
    let mut s = include_str!("../input.txt").lines().filter(|g| g.len() > 0).collect::<Vec<&str>>();
    s.push("[[2]]");
    s.push("[[6]]");
    let result = s.iter().sorted_by(|left, right| compare_lists(right.as_bytes(), left.as_bytes())).collect::<Vec<&&str>>();
    #[cfg(debug_assertions)]
    {
        println!("{:?}", s);
        println!("{:?}", result);
    }
    print!("{} ", (result.iter().position(|s| **s == "[[2]]").unwrap() + 1) * (result.iter().position(|s| **s == "[[6]]").unwrap() + 1));
}
