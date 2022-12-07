pub fn main() {
    let b = include_bytes!("../input.txt");
    let mut wd = Vec::<u32>::with_capacity(32);
    wd.push(0);
    let mut sizes = Vec::<u32>::with_capacity(256);

    b.split(|x| x == &b'\n')
        .filter(|l| l.len() > 2)
        .for_each(|l| match &l[0..3] {
            b"$ c" => {
                match l[5] {
                    b'.' => {
                        let s = wd.pop().unwrap();
                        sizes.push(s);
                        *wd.last_mut().unwrap() += s;
                    }
                    _ => {
                        wd.push(0);
                    }
                }
            }
            _ if l[0] >= b'0' && l[0] <= b'9' => {
                *wd.last_mut().unwrap() += atoi::atoi::<u32>(l.split(|b| b == &b' ').next().unwrap()).unwrap()
            }
            _ => assert!(true),
        });

    let mut size = 0;
    wd.into_iter().rev().for_each(|x| {
        let s = x + size;
        sizes.push(s);
        size = s;
    });

    let free_space = 70000000 - sizes.last().unwrap();
    let to_delete = 30000000 - free_space;
    // println!("{} {} {}", total_size, free_space, to_delete);

    println!(
        "{}",
        sizes.into_iter().filter(|s| *s >= to_delete).min().unwrap()
    );
}
