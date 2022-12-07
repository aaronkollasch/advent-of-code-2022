use std::collections::HashMap;

pub fn main() {
    let s = "\n".to_owned() + include_str!("../input.txt");
    let mut wd = Vec::<&str>::new();
    let mut total_size = 0;
    let mut sizes = HashMap::<String, u32>::new();

    s.split("\n$ ")
        .filter(|g| g.len() > 2)
        // .take(10)
        .for_each(|g| match &g[0..2] {
            "cd" => match &g[3..] {
                "/" => {
                    wd.clear();
                }
                ".." => {
                    wd.pop();
                }
                _ => {
                    wd.push(&g[3..]);
                }
            },
            "ls" => {
                let s = g
                    .lines()
                    .skip(1)
                    .filter(|l| l.len() > 0 && &l[0..1] != "d")
                    .filter_map(|l| l.split_once(" ").unwrap().0.parse::<u32>().ok())
                    .sum::<u32>();
                // println!("{}\n{} {}", g, s, wd.join("/"));
                // if !sizes.contains_key(&wd.join("/")) { // use if ls commands are repeated
                total_size += s;
                // for i in 0..wd.len() { // minimal assumptions
                for i in 1..wd.len().min(4) { // assume depth is >0 and <4
                // for i in 2..wd.len().min(3) { // assume depth is 2
                    *sizes.entry(wd[..=i].join("/")).or_insert(0) += s;
                    // println!("{} +{} = {}", wd[..=i].join("/"), s, sizes[&wd[..=i].join("/")]);
                }
                // }
            }
            _ => assert!(true),
        });

    let free_space = 70000000 - total_size;
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
            .into_values()
            .filter(|s| *s >= to_delete)
            .min()
            .unwrap()
    );
}
