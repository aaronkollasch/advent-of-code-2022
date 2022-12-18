use rustc_hash::FxHashSet;
use std::cmp::{max, min};

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
    let s = include_str!("../input.txt");
    let (mut min_a, mut max_a) = (CubePos::MAX, CubePos::MIN);
    let (mut min_b, mut max_b) = (CubePos::MAX, CubePos::MIN);
    let (mut min_c, mut max_c) = (CubePos::MAX, CubePos::MIN);
    let mut cubes: FxHashSet<Pos3d> = Default::default();
    cubes.reserve(2400);
    s.lines().for_each(|l| {
        let mut i_num = 0;
        let mut acc = 0;
        let mut result = [0; 3];
        for b in l.as_bytes().iter() {
            match *b {
                b'0'..=b'9' => {
                    acc = acc * 10 + (b - b'0') as CubePos;
                }
                b',' => {
                    result[i_num] = acc * 2;
                    i_num += 1;
                    acc = 0;
                }
                _ => {}
            }
        }
        result[i_num] = acc * 2;
        min_a = min(min_a, result[0]);
        max_a = max(max_a, result[0]);
        min_b = min(min_b, result[1]);
        max_b = max(max_b, result[1]);
        min_c = min(min_c, result[2]);
        max_c = max(max_c, result[2]);
        cubes.insert((result[0], result[1], result[2]));
    });
    let mut sides: FxHashSet<Pos3d> = Default::default();
    sides.reserve(13000);
    cubes.iter().for_each(|(a, b, c)| {
        sides.extend(
            [
                (a + 1, *b, *c),
                (*a, *b + 1, *c),
                (*a, *b, c + 1),
                (a - 1, *b, *c),
                (*a, b - 1, *c),
                (*a, *b, *c - 1),
            ]
            .into_iter(),
        );
    });
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

    let cells: Vec<Vec<Vec<bool>>> = Vec::from_iter((min_c - 2..=max_c + 2).step_by(2).map(|c| {
        Vec::from_iter((min_b - 2..=max_b + 2).step_by(2).map(|b| {
            Vec::from_iter(
                (min_a - 2..=max_a + 2)
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
    next_positions.reserve(256);
    let mut new_positions: FxHashSet<Pos3d> = Default::default();
    next_positions.reserve(256);
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

    let mut sides2: FxHashSet<Pos3d> = Default::default();
    let mut num_cubes2 = 0;
    sides2.reserve(4096);
    contained_cube_iter.for_each(|(a, b, c)| {
        num_cubes2 += 1;
        sides2.extend(
            [
                (a + 1, b, c),
                (a, b + 1, c),
                (a, b, c + 1),
                (a - 1, b, c),
                (a, b - 1, c),
                (a, b, c - 1),
            ]
            .into_iter(),
        );
    });
    let num_neighbors2 = num_cubes2 * 6 - sides2.len();
    let num_surface2 = num_cubes2 * 6 - num_neighbors2 * 2;

    #[cfg(debug_assertions)]
    {
        println!("num cubes2: {}", num_cubes2);
        println!("num sides2: {}", sides2.len());
        println!("num neighbors2: {}", num_neighbors2);
        println!("num surface sides2: {}", num_surface2);
    }
    print!("{} ", num_surface - num_surface2);
}
