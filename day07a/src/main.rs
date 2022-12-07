pub fn main() {
    let b = include_bytes!("../input.txt");
    let mut wd = Vec::<u32>::with_capacity(32);
    wd.push(0);
    let mut total_size = 0;

    b.split(|x| x == &b'\n')
        .filter(|l| l.len() > 2)
        .for_each(|l| match l {
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
            _ if l[0] >= b'0' && l[0] <= b'9' => {
                *wd.last_mut().unwrap() += atoi::atoi::<u32>(l.split(|b| b == &b' ').next().unwrap()).unwrap()
            }
            _ => assert!(true),
        });

    let mut size = 0;
    wd.into_iter().rev().for_each(|x| {
        let s = x + size;
        if s <= 100_000 {
            total_size += s;
        }
        size = s;
    });
    println!("{}", total_size);
}
