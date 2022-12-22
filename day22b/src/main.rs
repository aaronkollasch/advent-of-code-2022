use std::iter::repeat;

type Pos = (isize, isize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Instruction {
    dist: Option<isize>,
    rotation: Option<isize>,
}

fn rotate(facing: Pos, by: isize) -> Pos {
    match by.rem_euclid(4) {
        0 => facing,
        1 => (-facing.1, facing.0),
        2 => (-facing.0, -facing.1),
        3 => (facing.1, -facing.0),
        _ => unreachable!(),
    }
}

fn facing_angle_to_axes(facing: isize) -> Pos {
    match facing.rem_euclid(4) {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => unreachable!(),
    }
}

fn next_pos_facing(pos: Pos, facing: Pos, map: &Vec<Vec<u8>>, edges: &Vec<(Vec<Pos>, Vec<Pos>, isize, isize, isize)>) -> (Pos, Pos) {
    let mut next_pos = (pos.0 + facing.0, pos.1 + facing.1);
    let mut next_facing = facing;
    for edge in edges.iter() {
        if let Some(i) = edge.0.iter().position(|p| *p == pos) {
            if facing_angle_to_axes(edge.3) == facing {
                (next_pos, next_facing) = (edge.1[i], rotate(facing, (edge.4 + 2) - edge.3));
                println!("cross edge: {} {} ({} {}) -> {} {} ({} {})", pos.0, pos.1, facing.0, facing.1, next_pos.0, next_pos.1, next_facing.0, next_facing.1);
            }
        } else if let Some(i) = edge.1.iter().position(|p| *p == pos) {
            if facing_angle_to_axes(edge.4) == facing {
                (next_pos, next_facing) = (edge.0[i], rotate(facing, (edge.3 + 2) - edge.4));
                println!("cross edge: {} {} ({} {}) -> {} {} ({} {})", pos.0, pos.1, facing.0, facing.1, next_pos.0, next_pos.1, next_facing.0, next_facing.1);
            }
        }
    }
    (next_pos, next_facing)
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut pos = (s.iter().position(|b| *b == b'.').unwrap() as isize, 0);
    let mut facing: (isize, isize) = (1, 0);
    let w = s
        .split(|b| *b == b'\n')
        .take_while(|l| !l.is_empty())
        .map(|l| l.len())
        .max()
        .unwrap();
    let map = s
        .split(|b| *b == b'\n')
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.iter()
                .take(l.len())
                .chain(repeat(&b' ').take(w - l.len()))
                .copied()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let h = map.len();
    // let l = 4isize;
    // let edges: Vec<(Vec<Pos>, Vec<Pos>, isize, isize, isize)> = vec![
    //     (repeat(0).take(l as usize).zip(l*2..l*3).collect(), repeat(l).take(l as usize).zip((0..l).rev()).collect(), 2, 3, 3),
    //     ((l..l*2).zip(repeat(l).take(l as usize)).collect(), repeat(l*2).take(l as usize).zip(0..l).collect(), 1, 3, 1),
    //     (repeat(l*3-1).take(l as usize).zip(l..l*2).collect(), (l*3..l*4).rev().zip(repeat(l*2).take(l as usize)).collect(), 1, 0, 3),
    //     (repeat(l*3-1).take(l as usize).zip(0..l).collect(), repeat(l*4-1).take(l as usize).zip(l*3..l*4).collect(), 2, 0, 2),
    //     (repeat(0).take(l as usize).zip(l..l*2).collect(), (l*3..l*4).rev().zip(repeat(l*3-1).take(l as usize)).collect(), 3, 2, 1),
    //     ((0..l).zip(repeat(l*2-1).take(l as usize)).collect(), (l*2..l*3).rev().zip(repeat(l*3-1).take(l as usize)).collect(), 2, 1, 1),
    //     ((l..l*2).zip(repeat(l*2-1).take(l as usize)).collect(), repeat(l*2).take(l as usize).zip((l*2..l*3).rev()).collect(), 1, 1, 2),
    // ];
    let l = 50isize;
    let edges: Vec<(Vec<Pos>, Vec<Pos>, isize, isize, isize)> = vec![
        ((0..l).zip(repeat(l*2).take(l as usize)).collect(), repeat(l).take(l as usize).zip(l..l*2).collect(), 3, 3, 2),
        (repeat(0).take(l as usize).zip(l*2..l*3).collect(), repeat(l).take(l as usize).zip((0..l).rev()).collect(), 2, 2, 2),
        (repeat(0).take(l as usize).zip(l*3..l*4).collect(), (l..l*2).zip(repeat(0).take(l as usize)).collect(), 1, 2, 3),
        ((0..l).zip(repeat(l*4-1).take(l as usize)).collect(), (l*2..l*3).zip(repeat(0).take(l as usize)).collect(), 0, 1, 3),
        (repeat(l-1).take(l as usize).zip(l*3..l*4).collect(), (l..l*2).zip(repeat(l*3-1).take(l as usize)).collect(), 1, 0, 1),
        (repeat(l*2-1).take(l as usize).zip(l*2..l*3).collect(), repeat(l*3-1).take(l as usize).zip((0..l).rev()).collect(), 2, 0, 0),
        (repeat(l*2-1).take(l as usize).zip(l..l*2).collect(), (l*2..l*3).zip(repeat(l-1).take(l as usize)).collect(), 1, 0, 1),
    ];
    let mut path: Vec<Instruction> = Vec::new();
    let mut acc = 0;
    s.split(|b| *b == b'\n')
        .rev()
        .nth(1)
        .unwrap()
        .iter()
        .for_each(|b| match b {
            b'0'..=b'9' => {
                acc = acc * 10 + (b - b'0') as isize;
            }
            b'R' => {
                path.push(Instruction {
                    dist: Some(acc),
                    rotation: None,
                });
                path.push(Instruction {
                    dist: None,
                    rotation: Some(1),
                });
                acc = 0;
            }
            b'L' => {
                path.push(Instruction {
                    dist: Some(acc),
                    rotation: None,
                });
                path.push(Instruction {
                    dist: None,
                    rotation: Some(-1),
                });
                acc = 0;
            }
            _ => unreachable!(),
        });
    path.push(Instruction {
        dist: Some(acc),
        rotation: None,
    });
    println!("{} {}", map.len(), map[0].len());
    println!(
        "pos: ({}, {}), facing ({} {})",
        pos.0, pos.1, facing.0, facing.1
    );
    for ins in path.iter() {
        println!("{:?}", ins);
        match (ins.dist, ins.rotation) {
            (Some(dist), None) => {
                for _step in 0..dist {
                    let (next_pos, next_facing) = next_pos_facing(pos, facing, &map, &edges);
                    // println!("next_pos: {} {}, next_facing: {} {}", next_pos.0, next_pos.1, next_facing.0, next_facing.1);
                    match map[next_pos.1 as usize][next_pos.0 as usize] {
                        b'#' => { break; }
                        b'.' => { (pos, facing) = (next_pos, next_facing); }
                        _ => { panic!("unexpected wall as next character"); }
                    }
                    // println!("pos: {} {}, facing: {} {}", pos.0, pos.1, facing.0, facing.1);
                }
            }
            (None, Some(rot)) => {
               (facing.0, facing.1) = rotate((facing.0, facing.1), rot);
            },
            _ => unreachable!(),
        }
        println!(
            "pos: ({}, {}), facing ({} {})",
            pos.0, pos.1, facing.0, facing.1
        );
    }
    let facing_dir = facing.0.abs() * (1 - facing.0) + facing.1.abs() * (2 - facing.1);
    println!(
        "pos: ({}, {}), facing ({} {}) = {}",
        pos.0 + 1, pos.1 + 1, facing.0, facing.1, facing_dir
    );
    print!("{} ", 1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + facing_dir);
}
