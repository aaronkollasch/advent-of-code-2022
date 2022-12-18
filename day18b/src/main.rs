use rustc_hash::FxHashSet;

type CubePos = i32;
type Pos3d = (CubePos, CubePos, CubePos);

struct Map {
    pub contents: Vec<Vec<Vec<bool>>>,
    pub w: CubePos,
    pub h: CubePos,
    pub d: CubePos,
}

impl Map {
    #[inline]
    pub fn get_val(&self, pos: Pos3d) -> bool {
        self.contents[pos.2 as usize][pos.1 as usize][pos.0 as usize]
    }

    #[inline]
    pub fn set_val(&mut self, pos: Pos3d, val: bool) {
        self.contents[pos.2 as usize][pos.1 as usize][pos.0 as usize] = val;
    }

    #[inline]
    pub fn iter_unset_neighbors(&self, pos: Pos3d) -> impl Iterator<Item = Pos3d> + '_ {
        [
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
        ]
        .iter()
        .filter_map(move |&p| {
            let new_pos = (pos.0 + p.0, pos.1 + p.1, pos.2 + p.2);
            if new_pos.0 >= 0
                && new_pos.0 < self.w
                && new_pos.1 >= 0
                && new_pos.1 < self.h
                && new_pos.2 >= 0
                && new_pos.2 < self.d
                && !self.get_val(new_pos)
            {
                Some(new_pos)
            } else {
                None
            }
        })
    }
}

pub fn main() {
    // let s = include_bytes!("../input.txt");
    // s.split(|b| *b == b'\n').for_each(|l| {
    // });
    let s = include_str!("../input.txt");
    let cubes = s.lines().filter(|l| !l.is_empty()).map(|l| {
        let result: Vec<CubePos> = l
            .split(',')
            .map(|w| w.parse::<CubePos>().unwrap() * 2)
            .collect();
        (result[0], result[1], result[2])
    }).collect::<Vec<_>>();
    let a_pos = cubes
        .iter()
        .map(|(a, _, _)| a)
        .collect::<FxHashSet<_>>();
    let (min_a, max_a) = (a_pos.iter().min().unwrap(), a_pos.iter().max().unwrap());
    let b_pos = cubes
        .iter()
        .map(|(_, b, _)| b)
        .collect::<FxHashSet<_>>();
    let (min_b, max_b) = (b_pos.iter().min().unwrap(), b_pos.iter().max().unwrap());
    let c_pos = cubes
        .iter()
        .map(|(_, _, c)| c)
        .collect::<FxHashSet<_>>();
    let (min_c, max_c) = (c_pos.iter().min().unwrap(), c_pos.iter().max().unwrap());
    let sides = cubes
        .iter()
        .flat_map(|(a, b, c)| {
            [
                (a + 1, *b, *c),
                (*a, *b + 1, *c),
                (*a, *b, c + 1),
                (a - 1, *b, *c),
                (*a, b - 1, *c),
                (*a, *b, *c - 1),
            ]
            .into_iter()
        })
        .collect::<FxHashSet<_>>();
    let num_cubes = cubes.len();
    let num_neighbors = num_cubes * 6 - sides.len();
    let num_surface = num_cubes * 6 - num_neighbors * 2;

    #[cfg(debug_assertions)]
    {
        println!("num cubes: {}", num_cubes);
        println!("num neighbors: {}", num_neighbors);
        println!("num sides: {}", num_cubes * 6);
        println!("num contacting sides: {}", num_neighbors * 2);
        println!("num surface sides: {}", num_surface);
    }

    let cubes = cubes.iter().collect::<FxHashSet<_>>();

    let cells: Vec<Vec<Vec<bool>>> = Vec::from_iter((*min_c - 2..=*max_c + 2).step_by(2).map(|c| {
        Vec::from_iter((*min_b - 2..=*max_b + 2).step_by(2).map(|b| {
            Vec::from_iter(
                (*min_a - 2..=*max_a + 2)
                    .step_by(2)
                    .map(|a| cubes.contains(&(a, b, c))),
            )
        }))
    }));
    let (w, h, d) = (
        cells[0][0].len() as CubePos,
        cells[0].len() as CubePos,
        cells.len() as CubePos,
    );
    let mut map: Map = Map {
        contents: cells.clone(),
        w,
        h,
        d,
    };
    #[cfg(debug_assertions)]
    println!("w {} h {} d {}", w, h, d);
    let mut next_positions: FxHashSet<Pos3d> = Default::default();
    let mut new_positions: FxHashSet<Pos3d> = Default::default();
    let start: Pos3d = (1, 1, 1);
    next_positions.insert(start);
    let start: Pos3d = (w - 1, h - 1, d - 1);
    next_positions.insert(start);

    while !next_positions.is_empty() {
        for p in next_positions.iter() {
            map.set_val(*p, true);
        }
        new_positions.extend(
            next_positions
                .drain()
                .flat_map(|p| map.iter_unset_neighbors(p)),
        );
        (next_positions, new_positions) = (new_positions, next_positions);
    }
    #[cfg(debug_assertions)]
    for (v12, v22) in cells.iter().zip(map.contents.iter()) {
        for (v11, v21) in v12.iter().zip(v22.iter()) {
            for b in v11.iter() {
                print!("{}", if *b { '#' } else { ' ' });
            }
            print!("  ->  ");
            for b in v21.iter() {
                print!("{}", if *b { '#' } else { ' ' });
            }
            println!();
        }
        println!("------------------------------------------------");
    }
    let contained_cube_iter = map.contents.iter().enumerate().flat_map(|(c, v2)| {
        v2.iter().enumerate().flat_map(move |(b, v1)| {
            v1.iter()
                .enumerate()
                .filter(|(_a, v)| !**v)
                .map(move |(a, _v)| (a as CubePos * 2, b as CubePos * 2, c as CubePos * 2))
        })
    });

    let sides2 = contained_cube_iter
        .clone()
        .flat_map(|(a, b, c)| {
            [
                (a + 1, b, c),
                (a, b + 1, c),
                (a, b, c + 1),
                (a - 1, b, c),
                (a, b - 1, c),
                (a, b, c - 1),
            ]
            .into_iter()
        })
        .collect::<FxHashSet<_>>();
    let num_cubes2 = contained_cube_iter.clone().count();
    let num_neighbors2 = num_cubes2 * 6 - sides2.len();
    let num_surface2 = num_cubes2 * 6 - num_neighbors2 * 2;

    #[cfg(debug_assertions)]
    {
        println!("num cubes2: {}", num_cubes2);
        println!("num neighbors2: {}", num_neighbors2);
        println!("num surface sides2: {}", num_surface2);
    }
    print!("{}", num_surface - num_surface2);
}
