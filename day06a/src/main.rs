use std::str;
use std::cmp;

const WINDOW: usize = 3;

pub fn main() {
    let b = include_bytes!("../input.txt");
    let mut buf: [u8; WINDOW] = [0; WINDOW];
    buf.clone_from_slice(&b[0..WINDOW]);
    let mut idx: usize = 0;
    let mut skip_count = WINDOW;
    let mut match_idx = 0;
    for (i, c) in b.iter().enumerate() {
        // println!("{} {} {} {}", i, c, skip_count, str::from_utf8(&buf).unwrap());
        for j in 0..WINDOW {
            if *c == buf[(idx+j) % WINDOW] {
                skip_count = cmp::max(skip_count, j+1)
            }
        }

        // if *c == buf[(idx + 2) % 3] {
        //     skip_count = 3;
        // } else if *c == buf[(idx + 1) % 3] {
        //     skip_count = cmp::max(skip_count, 2);
        // } else if *c == buf[idx] {
        //     skip_count = cmp::max(skip_count, 1);
        // } else if skip_count > 0 {
        //     skip_count -= 1;
        // }
        // println!("{} {} {} {}", i, c, skip_count, str::from_utf8(&buf).unwrap());
        if skip_count == 0 {
            match_idx = i;
            break;
        }
        if skip_count > 0 {
            skip_count -= 1;
        }
        buf[idx] = *c;
        idx = (idx + 1) % WINDOW;
    }
    println!("{} {}", match_idx+1, str::from_utf8(&b[match_idx-WINDOW..match_idx+1]).unwrap());
    // println!("{}", match_idx+1);
}
