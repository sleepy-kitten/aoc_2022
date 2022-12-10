use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day1, part1)]
pub fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .batching(|lines| {
            let mut acc = 0;
            while let Ok(line) = lines.next()?.parse::<u32>() {
                acc += line;
            }
            Some(acc)
        })
        .max()
        .unwrap()
}

#[aoc(day1, part1, split)]
pub fn solve_1_split(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(str::parse::<u32>)
                .filter_map(Result::ok)
                .sum()
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .batching(|lines| {
            let mut acc = 0;
            while let Ok(line) = lines.next()?.parse::<u32>() {
                acc += line;
            }
            Some(acc)
        })
        .fold([0; 3], |mut max, cal| {
            let min = max.iter_mut().min().unwrap();
            if *min < cal {
                *min = cal
            }
            max
        })
        .iter()
        .sum()
}
