use std::{collections::HashMap, cmp::{min, max}};
use kdam::tqdm;

type Number = isize;
type Pos = (Number, Number);

type Elf = Pos;

fn get_directions(dir: usize) -> [Pos; 3] {
    match dir % 4 {
        0 => [(1, -1), (0, -1), (-1, -1)],
        1 => [(1, 1), (0, 1), (-1, 1)],
        2 => [(-1, 1), (-1, 0), (-1, -1)],
        3 => [(1, 1), (1, 0), (1, -1)],
        _ => unreachable!(),
    }
}

fn print_elves(elves: &Vec<Elf>) {
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

    let mut elves: Vec<Elf> = Vec::with_capacity(256);
    s.split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(y, l)| {
            for (x, b) in l.iter().enumerate() {
                if *b == b'#' {
                    elves.push((x as Number, y as Number));
                }
            }
        });
    let mut proposed_pos: HashMap<Pos, usize> = HashMap::new();
    let mut proposed_elves: HashMap<usize, Pos> = HashMap::new();
    #[cfg(debug_assertions)]
    print_elves(&elves);
    for i_round in tqdm!(0..) {
        // first half
        for (i_elf, elf) in elves.iter().enumerate() {
            let neighbors = elves
                .iter()
                .filter_map(|e| {
                    let d = (e.0 - elf.0, e.1 - elf.1);
                    if d.0.abs() <= 1 && d.1.abs() <= 1 && !(d.0 == 0 && d.1 == 0) {
                        Some(d)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if !neighbors.is_empty() {
                let mut p: Option<Pos> = None;
                for dir in i_round..i_round + 4 {
                    let dirs = get_directions(dir);
                    if !dirs.into_iter().any(|d| neighbors.contains(&d)) {
                        p = Some(dirs[1]);
                        break;
                    }
                }
                if let Some(p) = p {
                    let p = (elf.0 + p.0, elf.1 + p.1);
                    proposed_elves.insert(i_elf, p);
                    proposed_pos
                        .entry(p)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }
        }
        #[cfg(debug_assertions)]
        println!("{:?}", proposed_pos);
        if proposed_pos.is_empty() {
            print!("{} ", i_round + 1);
            break;
        }
        // second half
        for (i_elf, new_pos) in proposed_elves.drain() {
            if proposed_pos[&new_pos] == 1 {
                if elves.contains(&new_pos) { panic!("overwriting elf at {:?}!", new_pos); }
                elves[i_elf] = new_pos;
            }
        }
        // reset
        proposed_pos.clear();
        #[cfg(debug_assertions)]
        print_elves(&elves);
    }
}
