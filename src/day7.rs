use std::cell::RefCell;

use aoc_runner_derive::aoc;

use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Node<'a, T> {
    size: u64,
    parent: Option<usize>,
    name: &'a str,
    variant: T,
}

#[derive(Debug, Default)]
pub struct Dir {
    dirs: Vec<usize>,
    files: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct File;

#[derive(Debug, Default)]
pub struct Tree<'a> {
    files: Vec<Node<'a, File>>,
    dirs: Vec<Node<'a, Dir>>,
    dir_sizes: Vec<usize>,
}

impl<'a> Tree<'a> {
    pub fn push_dir(&mut self, dir: Node<'a, Dir>) -> usize {
        let idx = self.dirs.len();
        self.dirs.push(dir);
        idx
    }
    pub fn push_file(&mut self, dir: Node<'a, File>) -> usize {
        let idx = self.files.len();
        self.files.push(dir);
        idx
    }
}

// #[aoc_generator(day7)]
pub fn generator<'a>(input: &'a str) -> Option<Tree<'a>> {
    let mut tree = Tree::<'a>::default();
    tree.push_dir(Node {
        size: 0,
        parent: None,
        name: "/",
        variant: Dir::default(),
    });

    let mut current_dir = 0;
    for line in input.lines().skip(1) {
        let mut spaced = line.split(' ');
        match spaced.next()? {
            "$" => match spaced.next()? {
                "cd" => match spaced.next()? {
                    ".." => {
                        current_dir = tree.dirs[current_dir].parent.unwrap_or(0);
                    }
                    "/" => {
                        current_dir = 0;
                    }
                    name => {
                        let node = tree.dirs[current_dir]
                            .variant
                            .dirs
                            .iter()
                            .find(|idx| tree.dirs[**idx].name == name)?;

                        current_dir = *node;
                    }
                },
                "ls" => {}
                _ => unreachable!(),
            },
            "dir" => {
                let name = spaced.next()?;

                let node = tree.push_dir(Node {
                    size: 0,
                    name,
                    parent: Some(current_dir),
                    variant: Dir::default(),
                });

                tree.dirs[current_dir].variant.dirs.push(node);
            }
            size => {
                let size = size.parse::<u64>().ok()?;
                let name = spaced.next()?;

                let node = tree.push_file(Node {
                    size,
                    name,
                    parent: Some(current_dir),
                    variant: File::default(),
                });
                tree.dirs[current_dir].variant.files.push(node);

                let ref_tree = RefCell::new(tree);

                std::iter::successors(Some(current_dir), |current_dir| {
                    ref_tree.borrow().dirs[*current_dir].parent
                })
                .for_each(|dir| {
                    ref_tree.borrow_mut().dirs[dir].size += size;
                });

                tree = ref_tree.into_inner();
            }
        }
    }
    let dirs = &tree.dirs;
    let sizes = &mut tree.dir_sizes;
    sizes.extend(
        std::iter::successors(Some(0), |prev| Some(prev + 1)).take_while(|idx| *idx < dirs.len()),
    );
    tree.dir_sizes
        .sort_by(|a, b| dirs[*a].size.cmp(&dirs[*b].size));

    Some(tree)
}

#[aoc(day7, part1)]
pub fn solve1(input: &str) -> u64 {
    let tree = generator(input).unwrap();

    tree.dirs
        .iter()
        .filter(|dir| dir.size <= 100000)
        .map(|dir| dir.size)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve2(input: &str) -> u64 {
    let tree = generator(input).unwrap();

    let free_space = 70000000 - tree.dirs[0].size;
    let missing_space = 30000000 - free_space;

    tree.dir_sizes
        .iter()
        .map(|idx| tree.dirs[*idx].size)
        .find(|size| *size > missing_space)
        .unwrap()
}
