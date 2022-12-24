use rustc_hash::FxHashSet as HashSet;
use itertools::iproduct;

type Number = usize;
type Pos = (Number, Number);

struct Hurricane {
    x: Number,
    y: Number,
    delta_x: Number,
    delta_y: Number,
}

const MOVEMENTS: [(isize, isize); 5] = [
    (0, 0),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

pub fn main() {
    let s = include_bytes!("../input.txt");
    let w = s.iter().position(|b| *b == b'\n').unwrap() - 2;
    let h = s.len() / (w + 3) - 2;

    #[cfg(debug_assertions)]
    println!("w: {}, h: {}", w, h);
    let mut hurricanes = s
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .skip(1)
        .take(h)
        .enumerate()
        .flat_map(move |(y, l)| {
            l[1..].iter().take(w).enumerate().filter_map(move |(x, b)| {
                match b {
                    b'>' => Some(Hurricane { x, y, delta_x: 1, delta_y: 0 }),
                    b'<' => Some(Hurricane { x, y, delta_x: w - 1, delta_y: 0 }),
                    b'v' => Some(Hurricane { x, y, delta_x: 0, delta_y: 1 }),
                    b'^' => Some(Hurricane { x, y, delta_x: 0, delta_y: h - 1 }),
                    _ => None,
                }
            })
        })
        .collect::<Vec<_>>();
    let mut map: Vec<bool> = Vec::from_iter(iproduct!(0..h, 0..w).map(|(y, x)| hurricanes.iter().any(|h| (h.x, h.y) == (x, y))));
    #[cfg(debug_assertions)]
    {
        for y in 0..h {
            println!("{:?}", String::from_iter(map[y * w..(y + 1) * w].iter().map(|b| if *b { '*' } else { ' ' })));
        }
        println!();
    }
    let mut positions: HashSet<Pos> = HashSet::with_capacity_and_hasher(1024, Default::default());
    let mut next_positions: HashSet<Pos> = positions.clone();
    let mut t = 0;
    while !positions.iter().any(|p| *p == (w - 1, h - 1)) {
        #[cfg(debug_assertions)]
        println!("{:?}", positions);
        for hurricane in hurricanes.iter_mut() {
            hurricane.x = (hurricane.x + hurricane.delta_x) % w;
            hurricane.y = (hurricane.y + hurricane.delta_y) % h;
        }
        map.clear();
        map.extend(iproduct!(0..h, 0..w).map(|(y, x)| hurricanes.iter().any(|h| (h.x, h.y) == (x, y))));
        #[cfg(debug_assertions)]
        {
            for y in 0..h {
                println!("{:?}", String::from_iter(map[y * w..(y + 1) * w].iter().map(|b| if *b { '*' } else { ' ' })));
            }
            println!();
        }
        if !map[0] {
            next_positions.insert((0, 0));
        }
        for position in positions.drain() {
            for movement in MOVEMENTS.into_iter() {
                let new_pos = (position.0.wrapping_add_signed(movement.0), position.1.wrapping_add_signed(movement.1));
                if new_pos.0 < w && new_pos.1 < h && !map[new_pos.1 * w + new_pos.0] {
                    next_positions.insert(new_pos);
                }
            }
        }

        (positions, next_positions) = (next_positions, positions);
        next_positions.clear();
        t += 1;
    }
    t += 1; // 1 extra minute to reach goal
    #[cfg(debug_assertions)]
    println!("t1: {}", t);
    for hurricane in hurricanes.iter_mut() {
        hurricane.x = (hurricane.x + hurricane.delta_x) % w;
        hurricane.y = (hurricane.y + hurricane.delta_y) % h;
    }
    positions.clear();
    while !positions.iter().any(|p| *p == (0, 0)) {
        #[cfg(debug_assertions)]
        println!("{:?}", positions);
        for hurricane in hurricanes.iter_mut() {
            hurricane.x = (hurricane.x + hurricane.delta_x) % w;
            hurricane.y = (hurricane.y + hurricane.delta_y) % h;
        }
        map.clear();
        map.extend(iproduct!(0..h, 0..w).map(|(y, x)| hurricanes.iter().any(|h| (h.x, h.y) == (x, y))));
        #[cfg(debug_assertions)]
        {
            for y in 0..h {
                println!("{:?}", String::from_iter(map[y * w..(y + 1) * w].iter().map(|b| if *b { '*' } else { ' ' })));
            }
            println!();
        }
        if !map[0] {
            next_positions.insert((w - 1, h - 1));
        }
        for position in positions.drain() {
            for movement in MOVEMENTS.into_iter() {
                let new_pos = (position.0.wrapping_add_signed(movement.0), position.1.wrapping_add_signed(movement.1));
                if new_pos.0 < w && new_pos.1 < h && !map[new_pos.1 * w + new_pos.0] {
                    next_positions.insert(new_pos);
                }
            }
        }

        (positions, next_positions) = (next_positions, positions);
        next_positions.clear();
        t += 1;
    }
    t += 1;
    for hurricane in hurricanes.iter_mut() {
        hurricane.x = (hurricane.x + hurricane.delta_x) % w;
        hurricane.y = (hurricane.y + hurricane.delta_y) % h;
    }
    #[cfg(debug_assertions)]
    println!("t2: {}", t);
    positions.clear();
    while !positions.iter().any(|p| *p == (w - 1, h - 1)) {
        #[cfg(debug_assertions)]
        println!("{:?}", positions);
        for hurricane in hurricanes.iter_mut() {
            hurricane.x = (hurricane.x + hurricane.delta_x) % w;
            hurricane.y = (hurricane.y + hurricane.delta_y) % h;
        }
        map.clear();
        map.extend(iproduct!(0..h, 0..w).map(|(y, x)| hurricanes.iter().any(|h| (h.x, h.y) == (x, y))));
        #[cfg(debug_assertions)]
        {
            for y in 0..h {
                println!("{:?}", String::from_iter(map[y * w..(y + 1) * w].iter().map(|b| if *b { '*' } else { ' ' })));
            }
            println!();
        }
        if !map[0] {
            next_positions.insert((0, 0));
        }
        for position in positions.drain() {
            for movement in MOVEMENTS.into_iter() {
                let new_pos = (position.0.wrapping_add_signed(movement.0), position.1.wrapping_add_signed(movement.1));
                if new_pos.0 < w && new_pos.1 < h && !map[new_pos.1 * w + new_pos.0] {
                    next_positions.insert(new_pos);
                }
            }
        }

        (positions, next_positions) = (next_positions, positions);
        next_positions.clear();
        t += 1;
    }
    t += 1; // 1 extra minute to reach goal

    println!("{} ", t);
}
