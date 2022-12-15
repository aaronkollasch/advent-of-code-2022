use itertools::Itertools;
use std::cmp::max;
use std::collections::HashSet;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let zones = s
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
            result
        })
        .collect::<Vec<_>>();
    let y = 2000000;
    let mut ranges_iter = zones
        .iter()
        .filter_map(|result| {
            let beacon_dist =
                result[0].abs_diff(result[2]) as isize + result[1].abs_diff(result[3]) as isize;
            let range_width = beacon_dist - result[1].abs_diff(y) as isize;
            if range_width < 0 {
                None
            } else {
                Some((result[0] - range_width, result[0] + range_width))
            }
        })
        .sorted_unstable();
    let (start, end) = ranges_iter.next().unwrap();
    let mut total = end - start + 1;
    let mut last_x = end;
    for (start, end) in ranges_iter {
        if start <= last_x + 1 {
            total += max(end - last_x, 0);
            last_x = max(last_x, end);
        } else {
            total += end - start + 1;
            last_x = end;
        }
    }
    #[cfg(debug_assertions)]
    println!("pre-total: {}", total);
    total -= zones
        .iter()
        .filter_map(|zone| if zone[3] == y { Some(zone[2]) } else { None })
        .collect::<HashSet<_>>()
        .iter()
        .count() as isize;
    print!("{} ", total);
}
