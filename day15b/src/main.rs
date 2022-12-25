pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut zones = Vec::with_capacity(32);
    s.split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .for_each(|l| {
            let mut has_number = false;
            let mut i_num = 0;
            let mut acc = 0;
            let mut sign = 1;
            let mut result = [0; 4];
            for b in l.iter() {
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
            zones.push((left, right, top, bottom));
        });

    let mut s_iter = zones
        .iter()
        .flat_map(|(_left, right, top, bottom)| {
            zones
                .iter()
                .filter(|(z_left, _z_right, z_top, z_bottom)| {
                    *z_left == *right && (*top <= *z_bottom || *z_top <= *bottom)
                })
                .map(|zone| zone.0)
        });
    let s = s_iter.next().unwrap();
    #[cfg(debug_assertions)]
    println!("s matches: {}", s_iter.count() + 1);
    let mut r_iter = zones
        .iter()
        .flat_map(|(left, right, _top, bottom)| {
            zones
                .iter()
                .filter(|(z_left, z_right, z_top, _z_bottom)| {
                    *z_top == *bottom && (*left <= *z_right || *z_left <= *right)
                })
                .map(|zone| zone.2)
        });
    let r = r_iter.next().unwrap();
    #[cfg(debug_assertions)]
    println!("r matches: {}", r_iter.count() + 1);
    #[cfg(debug_assertions)]
    println!("s: {} r: {}", s, r);
    let final_y = (s + r) / 2;
    let final_x = r - final_y;
    #[cfg(debug_assertions)]
    println!("x: {} y: {}", final_x, final_y);
    print!("{} ", 4000000 * final_x + final_y);
}
