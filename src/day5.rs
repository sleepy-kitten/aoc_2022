use std::{fmt::Write, marker::PhantomData, ops::Index, slice::SliceIndex};

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use derive_more::*;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Crate(pub u8);
#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Stack(pub Vec<Crate>);
#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Stacks(pub Vec<Stack>);

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl std::fmt::Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0 as char)
    }
}

impl std::fmt::Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.iter().map(|stack| stack.len()).max().unwrap();
        for idx in (0..len).rev() {
            for stack in self.iter() {
                if let Some(_crate) = stack.get(idx) {
                    write!(f, "{} ", _crate)?;
                } else {
                    write!(f, "    ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day5)]
pub fn generate(input: &str) -> (Stacks, Vec<Instruction>) {
    let mut input = input.split("\n\n");

    let stacks =
        input
            .next()
            .unwrap()
            .lines()
            .rev()
            .skip(1)
            .fold(Stacks(Vec::new()), |stacks, line| {
                Stacks(
                    line.as_bytes()
                        .iter()
                        .skip(1)
                        .step_by(4)
                        .zip(
                            stacks
                                .0
                                .into_iter()
                                .chain(std::iter::repeat_with(|| Stack(Vec::new()))),
                        )
                        .map(|(&_crate, mut stack)| {
                            if _crate != b' ' {
                                stack.push(Crate(_crate));
                            }
                            stack
                        })
                        .collect(),
                )
            });

    let instructions = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|num| num.parse::<usize>().ok())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .map(|(count, from, to)| Instruction {
            count,
            from: from - 1,
            to: to - 1,
        })
        .collect_vec();

    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn solve_1((stacks, instructions): &(Stacks, Vec<Instruction>)) -> String {
    let mut stacks = stacks.clone();
    for instruction in instructions {
        for _ in 0..instruction.count {
            let crate_ = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(crate_)
        }
    }

    stacks
        .0
        .iter()
        .map(|stack| stack.last().unwrap().0 as char)
        .collect::<String>()
}

#[aoc(day5, part2)]
pub fn solve_2((stacks, instructions): &(Stacks, Vec<Instruction>)) -> String {
    let mut stacks = stacks.clone();
    for instruction in instructions {
        let slice_index =
            (stacks[instruction.from].len() - instruction.count)..stacks[instruction.from].len();

        let removed = stacks[instruction.from].drain(slice_index).collect_vec();
        stacks[instruction.to].extend_from_slice(&removed);
    }

    stacks
        .0
        .iter()
        .map(|stack| stack.last().unwrap().0 as char)
        .collect::<String>()
}
