type Int = isize;

pub fn main() {
    let s = include_bytes!("../input.txt");
    let mut snafu_sum: Int = s
        .split(|b| *b == b'\n')
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
    let mut snafu_vec: Vec<u8> = Vec::with_capacity(32);
    while snafu_sum != 0 {
        let num = ((snafu_sum + 2) % 5) - 2;
        #[cfg(debug_assertions)]
        println!("{}\t{}", snafu_sum, num);
        let snafu_char = match num {
            0..=2 => b'0' + num as u8,
            -1 => b'-',
            -2 => b'=',
            _ => unreachable!(),
        };
        snafu_vec.push(snafu_char);
        snafu_sum -= num;
        snafu_sum /= 5;
    }
    snafu_vec.reverse();
    print!("{} ", String::from_utf8(snafu_vec).unwrap());
}
