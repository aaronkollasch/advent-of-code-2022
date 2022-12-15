use itertools::Itertools;
use std::collections::HashSet;
use std::cmp::max;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let zones = s.split(|b| *b == b'\n').filter(|l| l.len() > 0).map(|l| {
        let mut has_number = false;
        let mut i_num = 0;
        let mut acc = 0;
        let mut sign = 1;
        let mut result = [0; 4];
        for b in l.into_iter() {
            match b {
                b'-' => {
                    has_number = true;
                    sign = -1;
                }
                b'0'..=b'9' => {
                    has_number = true;
                    acc = acc * 10 + (b - b'0') as isize;
                }
                b' ' | b',' if has_number => {
                    result[i_num] = sign * acc;
                    i_num += 1;
                    has_number = false;
                    (sign, acc) = (1, 0);
                }
                _ => {}
            }
        }
        result[i_num] = sign * acc;
        result
    }).collect::<Vec<_>>();
    let y = 2000000;
    let mut ranges: Vec<(isize, isize)> = Vec::new();
    for (start, end) in zones.iter().map(|result| {
        let beacon_dist = result[0].abs_diff(result[2]) as isize + result[1].abs_diff(result[3]) as isize;
        let range_width = beacon_dist - result[1].abs_diff(y) as isize;
        (result[0] - range_width, result[0] + range_width)
    }).sorted() {
        if ranges.len() == 0 {
            ranges.push((start, end));
        } else {
            let last_range = ranges.last_mut().unwrap();
            if start <= last_range.1 + 1 {
                last_range.1 = max(last_range.1, end);
            } else {
                ranges.push((start, end));
            }
        }
    }
    let mut total = 0;
    for range in ranges.iter() {
        total += range.1 - range.0 + 1;
    }
    total -= zones.iter().filter_map(|zone| {
        if zone[3] == y { Some(zone[2]) }
        else { None }
    }).collect::<HashSet<_>>().iter().count() as isize;
    print!("{} ", total);
}
