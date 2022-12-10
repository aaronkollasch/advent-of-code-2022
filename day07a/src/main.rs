pub fn main() {
    let b = include_bytes!("../input.txt");
    let mut wd = Vec::<u32>::with_capacity(16);
    let mut total_size = 0;

    b.split(|x| x == &b'\n')
        .filter(|l| l.len() > 2)
        .for_each(|l| match l {
            _ if l[0] >= b'0' && l[0] <= b'9' => {
                *wd.last_mut().unwrap() += l
                    .iter()
                    .take_while(|x| **x != b' ')
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as u32);
            }
            b"$ cd .." => {
                let s = wd.pop().unwrap();
                if s <= 100_000 {
                    total_size += s;
                }
                *wd.last_mut().unwrap() += s;
            }
            _ if &l[0..3] == b"$ c" => {
                wd.push(0);
            }
            _ => {}
        });

    total_size += wd
        .into_iter()
        .rev()
        .scan(0, |size, s| Some(*size + s))
        .take_while(|s| *s <= 100_000)
        .sum::<u32>();
    print!("{} ", total_size);
}
