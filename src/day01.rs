pub const URL: &str = "https://adventofcode.com/2022/day/1/input";

pub fn solve1(text: &str) -> u32 {
    text.split("\n\n").map(
        |x| x.lines().map(|l| l.parse::<u32>().unwrap()).sum()
    ).max().unwrap()
}

pub fn solve2(text: &str) -> u32 {
    let mut elves_total: Vec<u32> = text.split("\n\n").map(
        |x| x.lines().map(|l| l.parse::<u32>().unwrap()).sum()
    ).collect();

    elves_total.sort_unstable();

    elves_total.iter().rev().take(3).sum()
}
