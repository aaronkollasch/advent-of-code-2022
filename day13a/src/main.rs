#[inline]
fn parse(b: &[u8]) -> i32 {
    b.iter().fold(0_i32, |acc, x| acc * 10 + (x - b'0') as i32)
}

fn compare_lists(left: &[u8], right: &[u8]) -> Option<bool> {
    #[cfg(debug_assertions)]
    {
        println!("compare");
        println!("{:?}", left);
        println!("{:?}", right);
    }
    let (mut depth_l, mut depth_r) = (0, 0);
    let left_iter = left[1..left.len() - 1]
        .split(move |b| match *b {
            b',' => depth_l == 0,
            b'[' => {
                depth_l += 1;
                false
            }
            b']' => {
                depth_l -= 1;
                false
            }
            _ => false,
        })
        .filter(|v| v.len() > 0);
    let len_l = left_iter.clone().count();
    let right_iter = right[1..right.len() - 1]
        .split(move |b| match *b {
            b',' => depth_r == 0,
            b'[' => {
                depth_r += 1;
                false
            }
            b']' => {
                depth_r -= 1;
                false
            }
            _ => false,
        })
        .filter(|v| v.len() > 0);
    let len_r = right_iter.clone().count();
    let mut result = left_iter.zip(right_iter).find_map(|(l, r)| {
        #[cfg(debug_assertions)]
        println!("-- {:?} {:?}", l, r);
        match (l[0], r[0]) {
            (b'[', b'[') => compare_lists(l, r),
            (b'[', _) => compare_lists(l, &[b"[", r, b"]"].concat()),
            (_, b'[') => compare_lists(&[b"[", l, b"]"].concat(), r),
            _ => {
                let (a, b) = (parse(l), parse(r));
                if a == b {
                    None
                } else {
                    Some(a <= b)
                }
            }
        }
    });
    #[cfg(debug_assertions)]
    println!("lengths {} {}", len_l, len_r);
    if len_l != len_r && result == None {
        result = Some(len_l < len_r)
    };
    #[cfg(debug_assertions)]
    match result {
        Some(ordered) => {
            println!("{}\n", if ordered { "ordered" } else { "not ordered" });
        }
        None => {
            println!("no decision");
        }
    }
    result
}

pub fn main() {
    let s = include_str!("../input.txt");
    let result = s
        .split("\n\n")
        .filter(|g| g.len() > 3)
        .enumerate()
        .map(|(i, g)| (i, g.split_once("\n").unwrap()))
        .filter_map(|(i, (left, right))| {
            let ordered = compare_lists(left.as_bytes(), right.as_bytes());
            #[cfg(debug_assertions)]
            println!(
                "{}\n",
                if ordered.unwrap() {
                    "CORRECT ORDER"
                } else {
                    "NOT ORDERED"
                }
            );
            if ordered.unwrap() {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    print!("{} ", result);
}