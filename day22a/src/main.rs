use common::Vec2;
use std::iter::repeat;

type Pos = Vec2<isize>;

#[non_exhaustive]
struct Dir;

impl Dir {
    pub const RIGHT: Pos = Pos { x: 1, y: 0 };
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Instruction {
    Move(isize),
    Rotate(isize),
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
    let h = map.len();
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
                    let mut next_pos = pos + facing;
                    #[cfg(debug_assertions)]
                    println!("next_pos: {}", next_pos);
                    let next_char = if next_pos.x >= 0
                        && next_pos.x < w as isize
                        && next_pos.y >= 0
                        && next_pos.y < h as isize
                    {
                        Some(map[next_pos.y as usize][next_pos.x as usize])
                    } else {
                        None
                    };
                    if next_char == Some(b'.') {
                        pos = next_pos;
                    } else if next_char.is_none() || next_char == Some(b' ') {
                        next_pos = pos - facing;
                        while next_pos.x >= 0
                            && next_pos.x < w as isize
                            && next_pos.y >= 0
                            && next_pos.y < h as isize
                            && map[next_pos.y as usize][next_pos.x as usize] != b' '
                        {
                            next_pos -= facing;
                        }
                        next_pos += facing;
                        if map[next_pos.y as usize][next_pos.x as usize] != b'#' {
                            pos = next_pos;
                        } else {
                            #[cfg(debug_assertions)]
                            println!("pos: {}", pos);
                            break;
                        }
                    } else {
                        #[cfg(debug_assertions)]
                        println!("pos: {}", pos);
                        break;
                    }
                }
            }
            Instruction::Rotate(rot) => match rot {
                1 => {
                    facing = Pos {
                        x: -facing.y,
                        y: facing.x,
                    }
                }
                -1 => {
                    facing = Pos {
                        x: facing.y,
                        y: -facing.x,
                    }
                }
                _ => unreachable!(),
            },
        }
        #[cfg(debug_assertions)]
        println!("pos: {}, facing {}", pos, facing);
    }
    let facing_dir = facing.x.abs() * (1 - facing.x) + facing.y.abs() * (1 - facing.y);
    #[cfg(debug_assertions)]
    println!("pos: {}, facing {} = {}", pos, facing, facing_dir);
    print!("{} ", 1000 * (pos.y + 1) + 4 * (pos.x + 1) + facing_dir);
}
