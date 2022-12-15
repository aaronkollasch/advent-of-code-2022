use itertools::Itertools;
use std::cmp::{max, min};
use std::thread;

const NUM_ZONES: usize = 32;
const I_MAX: isize = 4000000;
const NUM_THREADS: isize = 16;
const CHUNK_SIZE: isize = I_MAX / NUM_THREADS;

fn find_indices(zones: [(isize, isize, isize); NUM_ZONES], y_min: isize, y_max: isize) -> (isize, isize) {
    let mut final_y = isize::MIN;
    let mut final_x = isize::MIN;
    let mut y = y_min;
    'outer: while y <= y_max {
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
    (final_x, final_y)
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let zones: [_; NUM_ZONES] = s
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
    let handles: Vec<_> = (0..NUM_THREADS).map(|i_thread| {
        thread::spawn(move || {
            let y_min = i_thread * CHUNK_SIZE;
            let y_max = (i_thread + 1) * CHUNK_SIZE;
            find_indices(zones, y_min, y_max)
        })
    }).collect::<Vec<_>>();
    handles.into_iter().for_each(|handle| {
        let (x, y) = handle.join().unwrap();
        final_x = max(final_x, x);
        final_y = max(final_y, y);
    });
    #[cfg(debug_assertions)]
    println!("{} {}", final_x, final_y);
    print!("{} ", final_y + 4000000 * final_x + final_y);
}
