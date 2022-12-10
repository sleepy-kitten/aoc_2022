use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(derive_more::Deref)]
pub struct Grid(Vec<Vec<u8>>);

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in &self.0 {
            for x in y {
                write!(f, "{x}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
#[aoc_generator(day8)]
pub fn generate(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(str::as_bytes)
            .map(|line| line.iter().map(|num| num - b'0').collect())
            .collect(),
    )
}

#[aoc(day8, part1)]
pub fn solve_1(grid: &Grid) -> usize {
    let x_len = grid[0].len();
    let y_len = grid.len();

    let up_iter = (0..x_len).map(|x_idx| {
        grid.iter()
            .enumerate()
            .rev()
            .map(move |(y_idx, xs)| (y_idx, xs[x_idx]))
    });
    let down_iter = (0..x_len).map(|x_idx| {
        grid.iter()
            .enumerate()
            .map(move |(y_idx, xs)| (y_idx, xs[x_idx]))
    });
    let left_iter = grid
        .iter()
        .map(|inner| inner.iter().copied().enumerate().rev());
    let right_iter = grid.iter().map(|inner| inner.iter().copied().enumerate());

    let up_height_indices = height_indices_from_iter(up_iter);
    let down_height_indices = height_indices_from_iter(down_iter);
    let left_height_indices = height_indices_from_iter(left_iter);
    let right_height_indices = height_indices_from_iter(right_iter);

    let up_height_indices = up_height_indices.collect::<Vec<_>>();
    let down_height_indices = down_height_indices.collect::<Vec<_>>();
    let left_height_indices = left_height_indices.collect::<Vec<_>>();
    let right_height_indices = right_height_indices.collect::<Vec<_>>();

    let up_height_indices = &up_height_indices;
    let down_height_indices = &down_height_indices;
    let left_height_indices = &left_height_indices;
    let right_height_indices = &right_height_indices;

    grid.iter()
        .enumerate()
        .flat_map(|(y_idx, y)| {
            y.iter()
                .copied()
                .enumerate()
                .filter(move |&(x_idx, height)| {
                    let height = height as usize;

                    let up_heights = up_height_indices[x_idx];
                    let down_heights = down_height_indices[x_idx];
                    let left_heights = left_height_indices[y_idx];
                    let right_heights = right_height_indices[y_idx];

                    let is_up_hidden = up_heights
                        .iter()
                        .enumerate()
                        .skip(height)
                        .filter_map(|(height, idx)| idx.map(|idx| (height, idx)))
                        .any(|(other_height, index)| other_height >= height && index > y_idx);

                    let is_down_hidden = down_heights
                        .iter()
                        .enumerate()
                        .skip(height)
                        .filter_map(|(height, idx)| idx.map(|idx| (height, idx)))
                        .any(|(other_height, index)| other_height >= height && index < y_idx);

                    let is_left_hidden = left_heights
                        .iter()
                        .enumerate()
                        .skip(height)
                        .filter_map(|(height, idx)| idx.map(|idx| (height, idx)))
                        .any(|(other_height, index)| other_height >= height && index > x_idx);

                    let is_right_hidden = right_heights
                        .iter()
                        .enumerate()
                        .skip(height)
                        .filter_map(|(height, idx)| idx.map(|idx| (height, idx)))
                        .any(|(other_height, index)| other_height >= height && index < x_idx);

                    !(is_up_hidden && is_down_hidden && is_left_hidden && is_right_hidden)
                })
        })
        .count()
}

pub fn height_indices_from_iter(
    iter: impl Iterator<Item = impl Iterator<Item = (usize, u8)>>,
) -> impl Iterator<Item = [Option<usize>; 10]> {
    iter.map(|line| {
        let mut heights = [None; 10];

        for (index, height) in line {
            let height = height as usize;
            if heights[height].is_none() {
                heights[height] = Some(index);
            }
        }

        heights
    })
}

#[aoc(day8, part1, kaya)]
pub fn part1(input: &Grid) -> usize {
    let input = &input;
    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;

    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter(move |&(x, height)| {
                (edge_check(y, max_y)
                    || edge_check(x, max_x)
                    || !((0..=x - 1).rev().any(|i| input[y][i] >= *height)
                        && (x + 1..=max_x).any(|i| input[y][i] >= *height)
                        && (0..=y - 1).rev().any(|i| input[i][x] >= *height)
                        && (y + 1..=max_y).any(|i| input[i][x] >= *height)))
            })
        })
        .count()
}

fn edge_check(cord: usize, max: usize) -> bool {
    cord == 0 || cord == max
}
