const NUM_ZONES: usize = 32;

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
            let s = result[1] - result[0];
            let r = result[0] + result[1];
            let (left, right) = (s - beacon_dist - 1, s + beacon_dist + 1);
            let (top, bottom) = (r - beacon_dist - 1, r + beacon_dist + 1);
            (left, right, top, bottom)
        })
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();

    let s = zones
        .iter()
        .map(|(_left, right, _top, _bottom)| {
            zones
                .iter()
                .map(|zone| zone.0)
                .filter(|z_left| *z_left == *right)
        })
        .flatten()
        .next()
        .unwrap();
    let r = zones
        .iter()
        .map(|(_left, _right, _top, bottom)| {
            zones
                .iter()
                .map(|zone| zone.2)
                .filter(|z_top| *z_top == *bottom)
        })
        .flatten()
        .next()
        .unwrap();
    #[cfg(debug_assertions)]
    println!("s: {} r: {}", s, r);
    let final_y = (s + r) / 2;
    let final_x = r - final_y;
    #[cfg(debug_assertions)]
    println!("x: {} y: {}", final_x, final_y);
    print!("{} ", 4000000 * final_x + final_y);
}
