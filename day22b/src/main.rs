use common::Vec2;
use std::iter::repeat;
// todo: replace edges with FxHashMap of (pos, facing) -> (pos, facing)

type Pos = Vec2<isize>;
type PosTup = (isize, isize);

#[non_exhaustive]
struct Dir;

impl Dir {
    pub const UP: Pos = Pos { x: 0, y: -1 };
    pub const DOWN: Pos = Pos { x: 0, y: 1 };
    pub const LEFT: Pos = Pos { x: -1, y: 0 };
    pub const RIGHT: Pos = Pos { x: 1, y: 0 };
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Instruction {
    Move(isize),
    Rotate(isize),
}

fn rotate(facing: Pos, by: isize) -> Pos {
    match by.rem_euclid(4) {
        0 => facing,
        1 => Pos {
            x: -facing.y,
            y: facing.x,
        },
        2 => Pos {
            x: -facing.x,
            y: -facing.y,
        },
        3 => Pos {
            x: facing.y,
            y: -facing.x,
        },
        _ => unreachable!(),
    }
}

fn facing_angle_to_axes(facing: isize) -> Pos {
    match facing.rem_euclid(4) {
        0 => Dir::RIGHT,
        1 => Dir::DOWN,
        2 => Dir::LEFT,
        3 => Dir::UP,
        _ => unreachable!(),
    }
}

fn next_pos_facing(
    pos: Pos,
    facing: Pos,
    edges: &[(Vec<PosTup>, Vec<PosTup>, isize, isize)],
) -> (Pos, Pos) {
    let mut next_pos = pos + facing;
    let mut next_facing = facing;
    for edge in edges.iter() {
        if let Some(i) = edge.0.iter().position(|p| pos == *p) {
            if facing_angle_to_axes(edge.2) == facing {
                (next_pos, next_facing) =
                    (Pos::from(edge.1[i]), rotate(facing, (edge.3 + 2) - edge.2));
                #[cfg(debug_assertions)]
                println!(
                    "cross edge: {}, facing {} -> {}, facing {}",
                    pos, facing, next_pos, next_facing,
                );
            }
        } else if let Some(i) = edge.1.iter().position(|p| pos == *p) {
            if facing_angle_to_axes(edge.3) == facing {
                (next_pos, next_facing) =
                    (Pos::from(edge.0[i]), rotate(facing, (edge.2 + 2) - edge.3));
                #[cfg(debug_assertions)]
                println!(
                    "cross edge: {}, facing {} -> {}, facing {}",
                    pos, facing, next_pos, next_facing,
                );
            }
        }
    }
    (next_pos, next_facing)
}

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut pos = Pos {
        x: s.iter().position(|b| *b == b'.').unwrap() as isize,
        y: 0,
    };
    let mut facing: Pos = Dir::RIGHT;
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
    // // edges: stored as (Edge1, Edge2, facing1, facing2)
    // let l = 4isize;
    // #[rustfmt::skip]
    // let edges: Vec<(Vec<PosTup>, Vec<PosTup>, isize, isize)> = vec![
    //     (repeat(0).take(l as usize).zip(l*2..l*3).collect(), repeat(l).take(l as usize).zip((0..l).rev()).collect(), 3, 3),
    //     ((l..l*2).zip(repeat(l).take(l as usize)).collect(), repeat(l*2).take(l as usize).zip(0..l).collect(), 3, 1),
    //     (repeat(l*3-1).take(l as usize).zip(l..l*2).collect(), (l*3..l*4).rev().zip(repeat(l*2).take(l as usize)).collect(), 0, 3),
    //     (repeat(l*3-1).take(l as usize).zip(0..l).collect(), repeat(l*4-1).take(l as usize).zip(l*3..l*4).collect(), 0, 2),
    //     (repeat(0).take(l as usize).zip(l..l*2).collect(), (l*3..l*4).rev().zip(repeat(l*3-1).take(l as usize)).collect(), 2, 1),
    //     ((0..l).zip(repeat(l*2-1).take(l as usize)).collect(), (l*2..l*3).rev().zip(repeat(l*3-1).take(l as usize)).collect(), 1, 1),
    //     ((l..l*2).zip(repeat(l*2-1).take(l as usize)).collect(), repeat(l*2).take(l as usize).zip((l*2..l*3).rev()).collect(), 1, 2),
    // ];
    // edges: stored as (Edge1, Edge2, facing1, facing2)
    let l = 50isize;
    #[rustfmt::skip]
    let edges: Vec<(Vec<PosTup>, Vec<PosTup>, isize, isize)> = vec![
        ((0..l).zip(repeat(l*2).take(l as usize)).collect(), repeat(l).take(l as usize).zip(l..l*2).collect(), 3, 2),
        (repeat(0).take(l as usize).zip(l*2..l*3).collect(), repeat(l).take(l as usize).zip((0..l).rev()).collect(), 2, 2),
        (repeat(0).take(l as usize).zip(l*3..l*4).collect(), (l..l*2).zip(repeat(0).take(l as usize)).collect(), 2, 3),
        ((0..l).zip(repeat(l*4-1).take(l as usize)).collect(), (l*2..l*3).zip(repeat(0).take(l as usize)).collect(), 1, 3),
        (repeat(l-1).take(l as usize).zip(l*3..l*4).collect(), (l..l*2).zip(repeat(l*3-1).take(l as usize)).collect(), 0, 1),
        (repeat(l*2-1).take(l as usize).zip(l*2..l*3).collect(), repeat(l*3-1).take(l as usize).zip((0..l).rev()).collect(), 0, 0),
        (repeat(l*2-1).take(l as usize).zip(l..l*2).collect(), (l*2..l*3).zip(repeat(l-1).take(l as usize)).collect(), 0, 1),
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
                path.push(Instruction::Move(acc));
                path.push(Instruction::Rotate(1));
                acc = 0;
            }
            b'L' => {
                path.push(Instruction::Move(acc));
                path.push(Instruction::Rotate(-1));
                acc = 0;
            }
            _ => unreachable!(),
        });
    path.push(Instruction::Move(acc));
    #[cfg(debug_assertions)]
    println!("{} {}", map.len(), map[0].len());
    #[cfg(debug_assertions)]
    println!("pos: {}, facing {}", pos, facing);
    for ins in path.into_iter() {
        #[cfg(debug_assertions)]
        println!("{:?}", ins);
        match ins {
            Instruction::Move(dist) => {
                for _step in 0..dist {
                    let (next_pos, next_facing) = next_pos_facing(pos, facing, &edges);
                    match map[next_pos.y as usize][next_pos.x as usize] {
                        b'#' => {
                            break;
                        }
                        b'.' => {
                            (pos, facing) = (next_pos, next_facing);
                        }
                        _ => {
                            panic!("unexpected wall as next character");
                        }
                    }
                }
            }
            Instruction::Rotate(rot) => {
                facing = rotate(facing, rot);
            }
        }
        #[cfg(debug_assertions)]
        println!("pos: {}, facing {}", pos, facing);
    }
    let facing_dir = facing.x.abs() * (1 - facing.x) + facing.y.abs() * (2 - facing.y);
    #[cfg(debug_assertions)]
    println!("pos: {}, facing {} = {}", pos + (1, 1), facing, facing_dir);
    print!("{} ", 1000 * (pos.y + 1) + 4 * (pos.x + 1) + facing_dir);
}
