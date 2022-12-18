use std::collections::HashSet;

type CubePos = u8;

pub fn main() {
    // let s = include_bytes!("../input.txt");
    // s.split(|b| *b == b'\n').for_each(|l| {
    // });
    let s = include_str!("../input.txt");
    let mut cube_iter = s.lines().filter(|l| l.len() > 0).map(|l| {
        let result: Vec<CubePos> = l
            .split(',')
            .map(|w| w.parse::<CubePos>().unwrap() * 2)
            .collect();
        (result[0], result[1], result[2])
    });
    let a_pos = cube_iter.clone().map(|(a, _, _)| a).collect::<HashSet<_>>();
    let b_pos = cube_iter.clone().map(|(_, b, _)| b).collect::<HashSet<_>>();
    let c_pos = cube_iter.clone().map(|(_, _, c)| c).collect::<HashSet<_>>();
    let moved_cubes = cube_iter
        .clone()
        .map(|(a, b, c)| {
            [
                (a + 1, b + 1, c + 2),
                (a + 2, b + 1, c + 1),
                (a + 1, b + 2, c + 1),
                (a + 1, b + 1, c),
                (a + 1, b, c + 1),
                (a, b + 1, c + 1),
            ]
            .into_iter()
        })
        .flatten()
        .collect::<HashSet<_>>();
    let num_cubes = cube_iter.count();
    println!("num cubes: {}", num_cubes);

    let num_neighbors = num_cubes * 6 - moved_cubes.len();
    println!("num neighbors: {}", num_neighbors);

    println!("num sides: {}", num_cubes * 6);
    println!("num contacting sides: {}", num_neighbors * 2);
    println!(
        "num non-contacting sides: {}",
        num_cubes * 6 - num_neighbors * 2
    );

    println!("{} ", num_cubes * 6 - num_neighbors * 2);
}
