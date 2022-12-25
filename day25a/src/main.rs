use std::collections::VecDeque;

type Int = isize;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut snafu_sum: Int = s
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.iter()
                .fold(0, |acc, x| acc * 5 + match x {
                    b'0'..=b'2' => (*x - b'0') as Int,
                    b'-' => -1,
                    b'=' => -2,
                    _ => unreachable!(),
                })
        })
        .sum();
    #[cfg(debug_assertions)]
    println!("snafu sum: {}", snafu_sum);
    let mut snafu_vec: VecDeque<char> = VecDeque::with_capacity(32);
    let mut place: Int = 5;
    let mut last_place: Int = 1;
    while snafu_sum != 0 {
        let num = ((snafu_sum + 2 * last_place) % place) / last_place - 2;
        #[cfg(debug_assertions)]
        println!("{}\t{}\t{}", place, snafu_sum, num);
        let snafu_char = match num {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        };
        snafu_vec.push_front(snafu_char);
        snafu_sum -= num * last_place;
        last_place = place;
        place *= 5;
    }
    print!("{} ", snafu_vec.into_iter().collect::<String>());
}
