type Pos = (i32, i32);

const ROPE_LEN: usize = 10;
const VISITED_WIDTH: i32 = 512;
const START_POS: Pos = (VISITED_WIDTH / 2, VISITED_WIDTH / 2);
const VISITED_SIZE: usize = (VISITED_WIDTH * VISITED_WIDTH / usize::BITS as i32) as usize;
const USIZE_EXP: usize = usize::BITS.trailing_zeros() as usize;
const USIZE_MASK: usize = (usize::BITS - 1) as usize;

struct BitVec {
    vec: [usize; VISITED_SIZE],
}

impl BitVec {
    pub fn new() -> Self {
        Self { vec: [0usize; VISITED_SIZE] }
    }

    #[inline]
    pub fn set_bit(&mut self, pos: Pos) {
        let index = (pos.0 + pos.1 * VISITED_WIDTH) as usize;
        let word = &mut self.vec[index >> USIZE_EXP];
        let shift = index & USIZE_MASK;
        *word |= 1 << shift;
    }

    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.vec.iter().map(|u| u.count_ones()).sum::<u32>()
    }

    #[cfg(debug_assertions)]
    pub fn bits_pos(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.vec.len())
            .map(|i| (0..USIZE_EXP).map(move |j| (i, j)))
            .flatten()
            .map(|(i, j)| (i, j, self.vec[i]))
            .filter(|(_i, j, b)| (b >> j) & 0b1 > 0)
            .map(|(i, j, _b)| {
                (
                    ((i << USIZE_EXP) + j) % VISITED_WIDTH as usize,
                    ((i << USIZE_EXP) + j) / VISITED_WIDTH as usize,
                )
            })
    }
}

pub fn main() {
    let cmds = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| {
            match (
                l[0],
                l[2..].iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u8),
            ) {
                (b'U', l) => ((0, -1), l),
                (b'D', l) => ((0, 1), l),
                (b'L', l) => ((-1, 0), l),
                (_, l) => ((1, 0), l),
            }
        });
    let mut rope: [Pos; ROPE_LEN] = [START_POS; ROPE_LEN];
    let mut visited = BitVec::new();
    visited.set_bit(rope[0]);

    for (dir, dist) in cmds {
        for _i_step in 0..dist {
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;
            for j in 1..ROPE_LEN {
                let head = rope[j - 1];
                let tail = &mut rope[j];
                if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                    tail.0 += head.0.cmp(&tail.0) as i32;
                    tail.1 += head.1.cmp(&tail.1) as i32;
                    (j == ROPE_LEN - 1).then(|| visited.set_bit(*tail));
                } else {
                    break;
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    println!(
        "x=[{}, {}], y=[{}, {}]",
        visited.bits_pos().map(|(x, _y)| x).min().unwrap(),
        visited.bits_pos().map(|(x, _y)| x).max().unwrap(),
        visited.bits_pos().map(|(_x, y)| y).min().unwrap(),
        visited.bits_pos().map(|(_x, y)| y).max().unwrap(),
    );
    println!("{}", visited.count_ones());
}
