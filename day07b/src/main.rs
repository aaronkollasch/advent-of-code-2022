pub fn main() {
    let b = include_bytes!("../input.txt");
    let mut wd = Vec::<u32>::with_capacity(16);
    let mut sizes = Vec::<u32>::with_capacity(256);

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
                sizes.push(s);
                *wd.last_mut().unwrap() += s;
            }
            _ if &l[0..3] == b"$ c" => {
                wd.push(0);
            }
            _ => {}
        });

    sizes.extend(wd.into_iter().rev().scan(0, |size, s| Some(*size + s)));

    let free_space = 70000000 - sizes.last().unwrap();
    let to_delete = 30000000 - free_space;
    // println!("{} {} {}", total_size, free_space, to_delete);

    println!(
        "{}",
        sizes.into_iter().filter(|s| *s >= to_delete).min().unwrap()
    );
}
