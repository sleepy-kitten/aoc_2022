use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::ops::RangeInclusive;

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Vec<[RangeInclusive<u8>; 2]> {
    input
        .lines()
        .map(|range| {
            let mut ranges = range.split(',');
            let gen_range = |input: &str| -> RangeInclusive<u8> {
                let mut range = input.split('-');
                let start = range.next().unwrap().parse::<u8>().unwrap();
                let end = range.next().unwrap().parse::<u8>().unwrap();
                start..=end
            };
            [
                gen_range(ranges.next().unwrap()),
                gen_range(ranges.next().unwrap()),
            ]
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_1(input: &[[RangeInclusive<u8>; 2]]) -> usize {
    input
        .iter()
        .filter(|[a, b]| {
            (a.contains(b.start()) && a.contains(b.end()))
                || (b.contains(a.start()) && b.contains(a.end()))
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_2(input: &[[RangeInclusive<u8>; 2]]) -> usize {
    input
        .iter()
        .filter(|[a, b]| {
            (a.contains(b.start()) || a.contains(b.end()))
                || (b.contains(a.start()) || b.contains(a.end()))
        })
        .count()
}
