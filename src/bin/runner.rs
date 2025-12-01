use aoc2022::jobs;
use took::Timer;

fn main() {
    let timer = Timer::new();
    jobs().iter().for_each(|j| j.0());
    timer.took().describe("everything");
}
