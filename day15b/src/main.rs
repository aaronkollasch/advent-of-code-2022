use itertools::Itertools;
use std::cmp::{min, max};

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
    let mut final_y = 0;
    let mut final_x = 0;
    const Y_MAX: isize = 4000000;
    for y in 0..=Y_MAX {
        let mut ranges: Vec<(isize, isize)> = vec![(0, 0)];
        for (start, end) in zones.iter().filter_map(|result| {
            let beacon_dist = result[0].abs_diff(result[2]) as isize + result[1].abs_diff(result[3]) as isize;
            let range_width = beacon_dist - result[1].abs_diff(y) as isize;
            if range_width < 0 {
                None
            } else {
                Some((result[0] - range_width, result[0] + range_width))
            }
        }).sorted() {
            let last_range = ranges.last_mut().unwrap();
            if start <= last_range.1 + 1 {
                last_range.1 = min(max(last_range.1, end), Y_MAX);
            } else if start <= Y_MAX {
                ranges.push((min(start, Y_MAX), min(end, Y_MAX)));
            }
        }
        if *ranges.last().unwrap() != (0isize, Y_MAX) {
            final_y = y;
            for range in ranges.iter() {
                if final_x < range.0 {
                    break;
                }
                final_x = range.1 + 1;
            }
        }
    }
    print!("{} ", final_y + 4000000 * final_x + final_y);
}
