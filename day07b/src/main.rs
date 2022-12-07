use std::collections::HashMap;

pub fn main() {
    let s = "\n".to_owned() + include_str!("../input.txt");
    let mut wd = Vec::<u32>::new();
    let mut sizes = Vec::<u32>::with_capacity(256);

    s.split("\n$ ")
        .filter(|g| g.len() > 2)
        .for_each(|g| match &g[0..1] {
            "c" => match &g[3..4] {
                "." => {
                    sizes.push(wd.pop().unwrap());
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

    wd.into_iter().rev().for_each(|x| sizes.push(x));
    let free_space = 70000000 - sizes.last().unwrap();
    let to_delete = 30000000 - free_space;
    // println!("{} {} {}", total_size, free_space, to_delete);
    // for (key, val) in sizes.iter() {
    //     if key.chars().filter(|c| *c == '/').count() <= 1 {
    //         println!("key: {key} val: {val}");
    //     }
    // }

    println!(
        "{}",
        sizes
            .into_iter()
            .filter(|s| *s >= to_delete)
            .min()
            .unwrap()
    );
}
