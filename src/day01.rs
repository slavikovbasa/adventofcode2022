pub const URL: &str = "https://adventofcode.com/2022/day/1/input";

pub fn solve1(text: &str) -> u32 {
    let elves = text.split("\n\n");
    let elves_total: Vec<u32> = elves.map(
        |x| x.lines().map(|l| l.parse::<u32>().unwrap()).sum()
    ).collect();

    *elves_total.iter().max().unwrap()
}

pub fn solve2(text: &str) -> u32 {
    let elves = text.split("\n\n");
    let mut elves_total: Vec<u32> = elves.map(
        |x| x.lines().map(|l| l.parse::<u32>().unwrap()).sum()
    ).collect();

    elves_total.sort_unstable();

    elves_total.iter().rev().take(3).sum()
}
