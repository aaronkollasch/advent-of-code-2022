const WINDOW: usize = 14;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .windows(WINDOW)
            .enumerate()
            // https://stackoverflow.com/a/46766782/653173
            .find(|(_, s)| !(1..s.len()).any(|i| s[i..].contains(&s[i - 1])))
            .unwrap()
            .0
            + WINDOW
    );
}

// pub fn main() {
//     let b = include_bytes!("../input.txt").map(|c| c - b'a');
//     let mut counts: [u8; 26] = [0; 26];
//     let mut b_iter = b.windows(WINDOW).enumerate();
//     let mut has_dup = false;
//     for c in b_iter.next().unwrap().1.iter().skip(1) {
//         counts[*c as usize] += 1;
//         has_dup |= counts[*c as usize] > 1;
//     }
//     for (i, w) in b_iter {
//         counts[w[WINDOW - 1] as usize] += 1;
//         has_dup |= counts[w[WINDOW - 1] as usize] > 1;

//         // println!("{} {} {} {:?}", i+1, has_dup, std::str::from_utf8(&w.iter().map(|c| c + b'a').collect::<Vec<u8>>()).unwrap(), counts);
//         if !has_dup {
//             println!("{}", i + WINDOW);
//             break;
//         }

//         counts[w[0] as usize] -= 1;
//         if counts[w[0] as usize] == 1 {
//             has_dup = counts.iter().any(|&c| c > 1)
//         }
//     }
// }
