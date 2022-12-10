use aoc_runner_derive::aoc;
use bitmaps::Bitmap;
use itertools::Itertools;

fn char_index(char: u8) -> u8 {
    if char.is_ascii_lowercase() {
        char - b'a'
    } else {
        char - b'A' + (b'a'..=b'z').len() as u8
    }
}

#[aoc(day3, part1)]
pub fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.as_bytes().split_at(line.len() / 2);

            let mut map = Bitmap::<52>::new();

            left.iter().copied().map(char_index).for_each(|index| {
                map.set(index as usize, true);
            });

            (right
                .iter()
                .copied()
                .map(char_index)
                .find(|char| map.get(*char as usize))
                .unwrap()
                + 1) as u32
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .map(str::as_bytes)
        .chunks(3)
        .into_iter()
        .map(|lines| {
            let mut map = [0_u32; 52];

            for line in lines {
                let mut local = [0_u32; 52];

                for idx in line.iter().copied().map(char_index) {
                    local[idx as usize] = 1;
                }
                for (local, map) in local.into_iter().zip(map.iter_mut()) {
                    *map += local
                }
            }

            map.into_iter().position(|num| num == 3).unwrap() as u32 + 1
        })
        .sum()
}
