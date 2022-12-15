use itertools::Itertools;
use std::cmp::{max, min};

pub fn main() {
    let s = include_bytes!("../input.txt");
    let zones: [_; 32] = s
        .split(|b| *b == b'\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
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
            let beacon_dist =
                result[0].abs_diff(result[2]) as isize + result[1].abs_diff(result[3]) as isize;
            (result[0], result[1], beacon_dist)
        })
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    let mut final_y = 0;
    let mut final_x = 0;
    const I_MAX: isize = 4000000;
    let mut y = 0;
    'outer: while y <= I_MAX {
        let mut last_x = 0;
        let mut min_overlap = isize::MAX;
        for (start, end) in zones
            .iter()
            .filter_map(|(s_x, s_y, beacon_dist)| {
                let range_width = beacon_dist - s_y.abs_diff(y) as isize;
                if range_width < 0 {
                    None
                } else {
                    Some((s_x - range_width, s_x + range_width))
                }
            })
            .sorted_unstable()
        {
            if start <= last_x + 1 {
                min_overlap = min(min_overlap, last_x - start + 1);
                last_x = min(max(last_x, end), I_MAX);
            } else {
                final_y = y;
                final_x = start - 1;
                break 'outer;
            }
        }
        y += (min_overlap + 1) / 2;
    }
    print!("{} ", final_y + 4000000 * final_x + final_y);
}
