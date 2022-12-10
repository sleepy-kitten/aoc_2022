use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn solve_1(input: &str) -> usize {
    input
        .as_bytes()
        .windows(4)
        .position(|chars| {
            chars
                .iter()
                .scan(bitmaps::Bitmap::<26>::new(), |set, char| {
                    Some(!set.set((*char - b'a') as usize, true))
                })
                .all(std::convert::identity)
        })
        .unwrap()
        + 4
}

#[aoc(day6, part2)]
pub fn solve_2(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .position(|chars| {
            chars
                .iter()
                .scan(bitmaps::Bitmap::<26>::new(), |set, char| {
                    Some(!set.set((*char - b'a') as usize, true))
                })
                .all(std::convert::identity)
        })
        .unwrap()
        + 14
}
