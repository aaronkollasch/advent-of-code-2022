use common::{parse, Vec2};

type Pos = Vec2<i32>;

#[non_exhaustive]
struct Dir;

impl Dir {
    pub const UP: Pos = Pos { x: 0, y: -1 };
    pub const DOWN: Pos = Pos { x: 0, y: 1 };
    pub const LEFT: Pos = Pos { x: -1, y: 0 };
    pub const RIGHT: Pos = Pos { x: 1, y: 0 };
}

const VISITED_WIDTH: i32 = 512;
const START_POS: Pos = Pos { x: VISITED_WIDTH / 2, y: VISITED_WIDTH / 2 };
const VISITED_SIZE: usize = (VISITED_WIDTH * VISITED_WIDTH / usize::BITS as i32) as usize;
const USIZE_EXP: usize = usize::BITS.trailing_zeros() as usize;
const USIZE_MASK: usize = (usize::BITS - 1) as usize;

struct BitVec {
    vec: [usize; VISITED_SIZE],
}

impl BitVec {
    pub fn new() -> Self {
        Self {
            vec: [0usize; VISITED_SIZE],
        }
    }

    #[inline]
    pub fn set_bit(&mut self, pos: Pos) {
        let index = (pos.x + pos.y * VISITED_WIDTH) as usize;
        let word = &mut self.vec[index >> USIZE_EXP];
        let shift = index & USIZE_MASK;
        *word |= 1 << shift;
    }

    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.vec.iter().map(|u| u.count_ones()).sum::<u32>()
    }

    #[cfg(debug_assertions)]
    pub fn bits_pos(&self) -> impl Iterator<Item = Vec2<usize>> + '_ {
        (0..self.vec.len())
            .flat_map(|i| (0..USIZE_EXP).map(move |j| (i, j)))
            .map(|(i, j)| (i, j, self.vec[i]))
            .filter(|(_i, j, b)| (b >> j) & 0b1 > 0)
            .map(|(i, j, _b)| {
                Vec2 {
                    x: ((i << USIZE_EXP) + j) % VISITED_WIDTH as usize,
                    y: ((i << USIZE_EXP) + j) / VISITED_WIDTH as usize,
                }
            })
    }
}

pub fn main() {
    let cmds = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| match (l[0], parse::<u8>(&l[2..])) {
            (b'U', l) => (Dir::UP, l),
            (b'D', l) => (Dir::DOWN, l),
            (b'L', l) => (Dir::LEFT, l),
            (_, l) => (Dir::RIGHT, l),
        });
    let (mut h, mut t): (Pos, Pos) = (START_POS, START_POS);
    let mut visited = BitVec::new();
    visited.set_bit(t);

    for (dir, dist) in cmds {
        for _i_step in 0..dist {
            h += dir;
            if h.x.abs_diff(t.x) > 1 || h.y.abs_diff(t.y) > 1 {
                t = h - dir;
                visited.set_bit(t)
            }
        }
    }

    #[cfg(debug_assertions)]
    println!(
        "x=[{}, {}], y=[{}, {}]",
        visited.bits_pos().map(|p| p.x).min().unwrap(),
        visited.bits_pos().map(|p| p.x).max().unwrap(),
        visited.bits_pos().map(|p| p.y).min().unwrap(),
        visited.bits_pos().map(|p| p.y).max().unwrap(),
    );
    print!("{} ", visited.count_ones());
}
