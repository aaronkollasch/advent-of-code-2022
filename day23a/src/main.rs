#[cfg(debug_assertions)]
use kdam::{tqdm, BarExt};
use rustc_hash::FxHashSet as HashSet;
use std::cmp::{max, min};

type Number = isize;
type Pos = (Number, Number);

const SURROUND: [Pos; 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

const DIRECTIONS: [[Pos; 3]; 4] = [
    [(1, -1), (0, -1), (-1, -1)],
    [(1, 1), (0, 1), (-1, 1)],
    [(-1, 1), (-1, 0), (-1, -1)],
    [(1, 1), (1, 0), (1, -1)],
];

#[cfg(debug_assertions)]
fn print_elves(elves: &HashSet<Pos>) {
    println!("{:?}", elves);
    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
    for elf in elves.iter() {
        min_x = min(min_x, elf.0);
        max_x = max(max_x, elf.0);
        min_y = min(min_y, elf.1);
        max_y = max(max_y, elf.1);
    }
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    println!("dims: {} {}", max_x - min_x + 1, max_y - min_y + 1);
    println!("area: {}", area);
    let mut map = vec![vec![" "; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for elf in elves.iter() {
        map[(elf.1 - min_y) as usize][(elf.0 - min_x) as usize] = "#";
    }
    for row in map {
        println!("{:?}", row.iter().cloned().collect::<String>());
    }
}

fn dest(elf: Pos, t: usize, elves: &HashSet<Pos>) -> Pos {
    if SURROUND
        .iter()
        .any(|d| elves.contains(&(elf.0 + d.0, elf.1 + d.1)))
    {
        for dir in t..t + 4 {
            let dirs = DIRECTIONS[dir % 4];
            if !dirs
                .iter()
                .any(|d| elves.contains(&(elf.0 + d.0, elf.1 + d.1)))
            {
                let dir = dirs[1];
                let p = (elf.0 + dir.0, elf.1 + dir.1);
                return p;
            }
        }
    }
    elf
}

pub fn main() {
    let s = include_bytes!("../input.txt");

    let mut elves: HashSet<Pos> = Default::default();
    elves.reserve(s.len());
    s.split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(y, l)| {
            for (x, b) in l.iter().enumerate() {
                if *b == b'#' {
                    elves.insert((x as Number, y as Number));
                }
            }
        });
    #[cfg(debug_assertions)]
    print_elves(&elves);
    #[cfg(debug_assertions)]
    let mut pb = tqdm!();
    for i_round in 0..10 {
        let mut new_elves = elves.clone();
        new_elves.clear();
        for &elf in elves.iter() {
            let new_pos = dest(elf, i_round, &elves);
            if new_pos == elf {
                new_elves.insert(elf);
            } else if !new_elves.insert(new_pos) {
                new_elves.remove(&new_pos);
                new_elves.insert(elf);
                new_elves.insert((new_pos.0 * 2 - elf.0, new_pos.1 * 2 - elf.1));
            }
        }
        elves = new_elves;
        // reset
        #[cfg(debug_assertions)]
        print_elves(&elves);
        #[cfg(debug_assertions)]
        pb.update(1);
    }
    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
    for elf in elves.iter() {
        min_x = min(min_x, elf.0);
        max_x = max(max_x, elf.0);
        min_y = min(min_y, elf.1);
        max_y = max(max_y, elf.1);
    }
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    print!("{} ", area - elves.len() as Number);
}
