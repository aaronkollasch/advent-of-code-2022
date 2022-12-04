pub fn main() {
    println!(
        "{}",
        include_str!("../target.txt")
            .lines()
            .map(|l| l
                .split(&[',', '-'][..])
                .filter_map(|s| s.parse::<u16>().ok())
                .collect::<Vec<u16>>())
            .filter(|v| v.len() == 4
                && ((v[0] <= v[2] && v[1] >= v[3]) || (v[0] >= v[2] && v[1] <= v[3])))
            .count()
    );
}
