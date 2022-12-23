use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
#[cfg(debug_assertions)]
use std::cmp::{min, max};
use kdam::tqdm;

type Number = isize;
type Pos = (Number, Number);

const DIRECTIONS: [[Pos; 3]; 4] = [
    [(1, -1), (0, -1), (-1, -1)],
    [(1, 1), (0, 1), (-1, 1)],
    [(-1, 1), (-1, 0), (-1, -1)],
    [(1, 1), (1, 0), (1, -1)],
];

#[cfg(debug_assertions)]
fn print_elves(elves: &HashSet<Pos>) {
    println!("{:?}", elves);
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
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
    let mut proposed_pos: HashMap<Pos, usize> = Default::default();
    proposed_pos.reserve(elves.len());
    let mut proposed_elves: HashMap<Pos, Pos> = Default::default();
    proposed_elves.reserve(elves.len());
    #[cfg(debug_assertions)]
    print_elves(&elves);
    for i_round in tqdm!(0..) {
        // first half
        proposed_elves.extend(elves.iter().filter_map(|elf| {
            let mut num_clear: u8 = 0;
            let mut p: Option<Pos> = None;
            for dir in (i_round..i_round + 4).rev() {
                let dirs = DIRECTIONS[dir % 4];
                if !dirs.iter().any(|d| elves.contains(&(elf.0 + d.0, elf.1 + d.1))) {
                    p = Some(dirs[1]);
                    num_clear += 1;
                }
            }
            match p {
                Some(p) if num_clear < 4 => {
                    let p = (elf.0 + p.0, elf.1 + p.1);
                    Some((*elf, p))
                }
                _ => { None }
            }
        }));
        for (_, new_pos) in proposed_elves.iter() {
            proposed_pos
                .entry(*new_pos)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        #[cfg(debug_assertions)]
        println!("{:?}", proposed_pos);
        if proposed_pos.is_empty() {
            print!("{} ", i_round + 1);
            break;
        }
        // second half
        for (elf, new_pos) in proposed_elves.drain() {
            if proposed_pos[&new_pos] == 1 {
                // extra check for debugging purposes:
                // if elves.contains(&new_pos) { panic!("overwriting elf at {:?}!", new_pos); }
                elves.remove(&elf);
                elves.insert(new_pos);
            }
        }
        // reset
        proposed_pos.clear();
        #[cfg(debug_assertions)]
        print_elves(&elves);
    }
}
