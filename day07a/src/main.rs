use std::collections::HashMap;

pub fn main() {
    let s = "\n".to_owned() + include_str!("../input.txt");
    let mut wd = Vec::<u32>::with_capacity(32);
    wd.push(0);
    // let mut sizes = Vec::<u32>::with_capacity(256);
    let mut total_size = 0;

    s.split("\n$ ")
        .filter(|g| g.len() > 2)
        // .take(10)
        .for_each(|g| match &g[0..1] {
            "c" => match &g[3..4] {
                "." => {
                    // sizes.push(wd.pop().unwrap());
                    let s = wd.pop().unwrap();
                    if s <= 100_000 { total_size += s; }
                }
                _ => {
                    wd.push(0);
                }
            },
            "l" => {
                let s = g
                    .lines()
                    .skip(1)
                    .filter(|l| l.len() > 0 && &l[0..1] != "d")
                    .filter_map(|l| l.split_once(" ").unwrap().0.parse::<u32>().ok())
                    .sum::<u32>();
                // println!("{}\n{} {}", g, s, wd.join("/"));
                wd.iter_mut().for_each(|x| *x += s);
            }
            _ => assert!(true),
        });

    println!(
        "{}",
        total_size
    );
}
