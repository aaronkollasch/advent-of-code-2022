use rustc_hash::FxHashSet;

type CubePos = i8;

pub fn main() {
    // let s = include_bytes!("../input.txt");
    // s.split(|b| *b == b'\n').for_each(|l| {
    // });
    let s = include_str!("../input.txt");
    let cubes = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let result: Vec<CubePos> = l
                .split(',')
                .map(|w| w.parse::<CubePos>().unwrap() * 2)
                .collect();
            (result[0], result[1], result[2])
        })
        .collect::<Vec<_>>();
    let moved_cubes = cubes
        .iter()
        .flat_map(|(a, b, c)| {
            [
                (a + 1, *b, *c),
                (*a, b + 1, *c),
                (*a, *b, c + 1),
                (a - 1, *b, *c),
                (*a, b - 1, *c),
                (*a, *b, c - 1),
            ]
            .into_iter()
        })
        .collect::<FxHashSet<_>>();
    let num_cubes = cubes.len();
    let num_neighbors = num_cubes * 6 - moved_cubes.len();
    let num_surface = num_cubes * 6 - num_neighbors * 2;

    #[cfg(debug_assertions)]
    {
        println!("num cubes: {}", num_cubes);
        println!("num neighbors: {}", num_neighbors);
        println!("num sides: {}", num_cubes * 6);
        println!("num contacting sides: {}", num_neighbors * 2);
        println!("num surface sides: {}", num_surface);
    }

    print!("{} ", num_surface);
}
