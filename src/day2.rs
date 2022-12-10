use aoc_runner_derive::aoc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Choice(pub u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Outcome(pub u8);

impl Choice {
    const ROCK: Self = Choice(0);
    const PAPER: Self = Choice(1);
    const SCISSORS: Self = Choice(2);
}

impl Outcome {
    const LOSS: Self = Outcome(0);
    const TIE: Self = Outcome(1);
    const WIN: Self = Outcome(2);
}

impl Choice {
    pub const fn outcome(self, other: Self) -> Outcome {
        match () {
            _ if self.0 == other.0 => Outcome::TIE,
            _ if self.0 == (other.0 + 1) % 3 => Outcome::WIN,
            _ => Outcome::LOSS,
        }
    }
    pub const fn score(self, outcome: Outcome) -> u32 {
        (self.0 + outcome.0 * 3) as u32
    }
}

impl Outcome {
    pub fn choice(self, choice: Choice) -> Choice {
        match self {
            Outcome::LOSS => Choice((choice.0 + 2) % 3),
            Outcome::TIE => choice,
            Outcome::WIN => Choice((choice.0 + 1) % 2),
            _ => unreachable!(),
        }
    }
}

#[aoc(day2, part1)]
pub fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .map(str::as_bytes)
        .map(|line| {
            let you = Choice(line[0] - b'A');
            let opp = Choice(line[2] - b'X');
            let out = you.outcome(opp);

            you.score(out)
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .map(str::as_bytes)
        .map(|line| {
            let opp = Choice(line[0] - b'A');
            let out = Outcome(line[2] - b'X');
            let you = out.choice(opp);

            you.score(out)
        })
        .sum()
}
