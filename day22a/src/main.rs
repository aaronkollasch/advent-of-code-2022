use std::iter::repeat;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Instruction {
    dist: Option<isize>,
    rotation: Option<isize>,
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
    #[cfg(debug_assertions)]
    println!("{} {}", map.len(), map[0].len());
    #[cfg(debug_assertions)]
    println!(
        "pos: ({}, {}), facing ({} {})",
        pos.0, pos.1, facing.0, facing.1
    );
    for ins in path.iter() {
        #[cfg(debug_assertions)]
        println!("{:?}", ins);
        match (ins.dist, ins.rotation) {
            (Some(dist), None) => {
                for _step in 0..dist {
                    let mut next_pos = (pos.0 + facing.0, pos.1 + facing.1);
                    #[cfg(debug_assertions)]
                    println!("next_pos: {} {}", next_pos.0, next_pos.1);
                    let next_char = if next_pos.0 >= 0
                        && next_pos.0 < w as isize
                        && next_pos.1 >= 0
                        && next_pos.1 < h as isize
                    {
                        Some(map[next_pos.1 as usize][next_pos.0 as usize])
                    } else {
                        None
                    };
                    if next_char == Some(b'.') {
                        pos = next_pos;
                    } else if next_char.is_none() || next_char == Some(b' ') {
                        next_pos = (pos.0 - facing.0, pos.1 - facing.1);
                        while next_pos.0 >= 0
                            && next_pos.0 < w as isize
                            && next_pos.1 >= 0
                            && next_pos.1 < h as isize
                            && map[next_pos.1 as usize][next_pos.0 as usize] != b' '
                        {
                            next_pos = (next_pos.0 - facing.0, next_pos.1 - facing.1);
                        }
                        next_pos = (next_pos.0 + facing.0, next_pos.1 + facing.1);
                        if map[next_pos.1 as usize][next_pos.0 as usize] != b'#' {
                            pos = next_pos;
                        } else {
                            #[cfg(debug_assertions)]
                            println!("pos: {} {}", pos.0, pos.1);
                            break;
                        }
                    } else {
                        #[cfg(debug_assertions)]
                        println!("pos: {} {}", pos.0, pos.1);
                        break;
                    }
                }
            }
            (None, Some(rot)) => match rot {
                1 => (facing.0, facing.1) = (-facing.1, facing.0),
                -1 => (facing.0, facing.1) = (facing.1, -facing.0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        #[cfg(debug_assertions)]
        println!(
            "pos: ({}, {}), facing ({} {})",
            pos.0, pos.1, facing.0, facing.1
        );
    }
    let facing_dir = facing.0.abs() * (1 - facing.0) + facing.1.abs() * (1 - facing.1);
    #[cfg(debug_assertions)]
    println!(
        "pos: ({}, {}), facing ({} {}) = {}",
        pos.0 + 1,
        pos.1 + 1,
        facing.0,
        facing.1,
        facing_dir
    );
    print!("{} ", 1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + facing_dir);
}
