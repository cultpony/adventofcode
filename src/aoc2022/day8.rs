use itertools::Itertools;
use std::ops::Range;

use crate::{
    matrix::{DynMatrix, Matrix},
    *,
};

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let mut input = read_file_lines("aoc2022/day8.txt").await?;
    let mut matrix_lines: Vec<Vec<u8>> = Vec::new();

    while let Some(line) = input.next().await {
        let line = line
            .chars()
            .map(|x| x.to_digit(10).expect("unexpected character"))
            .map(|digit| {
                assert!(digit < 10);
                digit as u8
            });
        matrix_lines.push(line.collect());
    }

    let matrix: TreeMatrix = matrix_lines.into();

    debug!("Read in matrix: {matrix:#?}");

    let mut num_long_trees = 0;
    for x in 1..matrix.size_x() - 1 {
        for y in 1..matrix.size_y() - 1 {
            if matrix.only_shorter_any_dirs(x, y) {
                num_long_trees += 1;
            }
        }
    }

    assert!(num_long_trees > 0);

    num_long_trees += matrix.size_x() * 2;
    num_long_trees += matrix.size_y() * 2 - 4;

    Ok(Reportable {
        year: 2022,
        day: 8,
        part: 1.into(),
        result: TaskResult::Usize(num_long_trees),
    })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let mut input = read_file_lines("aoc2022/day8.txt").await?;
    let mut matrix_lines: Vec<Vec<u8>> = Vec::new();

    while let Some(line) = input.next().await {
        let line = line
            .chars()
            .map(|x| x.to_digit(10).expect("unexpected character"))
            .map(|digit| {
                assert!(digit < 10);
                digit as u8
            });
        matrix_lines.push(line.collect());
    }

    let matrix: TreeMatrix = matrix_lines.into();

    debug!("Read in matrix: {matrix:#?}");

    let mut best_scenic = 0;
    for x in 1..matrix.size_x() - 1 {
        for y in 1..matrix.size_y() - 1 {
            let ss = matrix.scenic_score(x, y);
            if ss > best_scenic {
                best_scenic = ss;
            }
        }
    }

    assert!(best_scenic > 0);

    Ok(Reportable {
        year: 2022,
        day: 8,
        part: 2.into(),
        result: TaskResult::Usize(best_scenic),
    })
}

#[derive(Debug)]
pub struct TreeMatrix(DynMatrix<u8>);

impl std::ops::Deref for TreeMatrix {
    type Target = DynMatrix<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TreeMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Vec<u8>>> for TreeMatrix {
    fn from(data: Vec<Vec<u8>>) -> Self {
        TreeMatrix(DynMatrix::from(data))
    }
}

impl TreeMatrix {
    /// returns true if all
    fn only_shorter(&self, x: usize, y: usize, dir: Direction) -> bool {
        let (tree_count, rem_trees, blocked) = self.tree_visible_count(x, y, dir);
        trace!("{dir:?}: Tree at {x} {y} has {tree_count} visible trees");
        if blocked {
            trace!("Has {rem_trees} non-visible trees, visible from edge");
            !blocked
        } else {
            trace!("Has {rem_trees} non-visible trees, not visible from edge");
            !blocked
        }
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        self.tree_visible_count(x, y, Direction::Up).0
            * self.tree_visible_count(x, y, Direction::Down).0
            * self.tree_visible_count(x, y, Direction::Left).0
            * self.tree_visible_count(x, y, Direction::Right).0
    }

    fn tree_visible_count(&self, x: usize, y: usize, dir: Direction) -> (usize, usize, bool) {
        let tree_size = *self.get(x, y).unwrap();
        let xo = x;
        let yo = y;
        let posi = dir.to_pos_iter(self, x, y);
        let mut tree_count: usize = 0;
        let mut stopped_count_at = None;
        let mut last_block_status = false;
        for (x, y) in posi {
            trace!("Checking position {x}/{y} against {xo}/{yo} tree size");
            let this_tree_size = *self.get(x, y).unwrap();
            tree_count += 1;
            if this_tree_size >= tree_size {
                if stopped_count_at.is_none() {
                    last_block_status = true;
                    trace!("{dir:?}: Tree at {x} {y} is longer ({this_tree_size}) than Tree at {xo} {yo} ({tree_size}), shadowed at {tree_count} trees in this range");
                    stopped_count_at = Some(tree_count);
                }
            } else if stopped_count_at.is_none() {
                last_block_status = false;
            }
        }
        trace!("{dir:?}: Tree at {x} {y} has {tree_count} visible trees");
        if stopped_count_at.is_none() {
            stopped_count_at = Some(tree_count);
        }
        let stopped_count_at = stopped_count_at.unwrap();
        let rem_tree = tree_count - stopped_count_at;
        (stopped_count_at, rem_tree, last_block_status)
    }

    pub fn only_shorter_any_dirs(&self, x: usize, y: usize) -> bool {
        self.only_shorter(x, y, Direction::Up)
            || self.only_shorter(x, y, Direction::Down)
            || self.only_shorter(x, y, Direction::Right)
            || self.only_shorter(x, y, Direction::Left)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn to_pos_iter(
        &self,
        tree: &TreeMatrix,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let size_x = tree.size_x();
        let size_y = tree.size_y();
        const fn dead_range_at(t: usize) -> Range<usize> {
            t..t + 1
        }

        fn range_pair_to_iter<
            I: IntoIterator<Item = usize>,
            I2: IntoIterator<Item = usize> + Clone,
        >(
            r: I,
            r2: I2,
        ) -> Vec<(usize, usize)>
        where
            <I2 as IntoIterator>::IntoIter: Clone + 'static,
        {
            let pos = r.into_iter().cartesian_product(r2);
            pos.collect_vec()
        }

        match self {
            Direction::Up => range_pair_to_iter((0..x).rev(), dead_range_at(y)).into_iter(),
            Direction::Down => range_pair_to_iter(x + 1..size_x, dead_range_at(y)).into_iter(),
            Direction::Right => range_pair_to_iter(dead_range_at(x), y + 1..size_y).into_iter(),
            Direction::Left => range_pair_to_iter(dead_range_at(x), (0..y).rev()).into_iter(),
        }
    }
}

#[cfg(test)]
fn test_matrix() -> TreeMatrix {
    let data = vec!["30373", "25512", "65332", "33549", "35390"];

    let mut data_out = Vec::new();
    for line in data {
        let line = line
            .chars()
            .map(|x| x.to_digit(10).expect("unexpected character"))
            .map(|digit| {
                assert!(digit < 10);
                digit as u8
            });
        data_out.push(line.collect_vec());
    }
    data_out.into()
}

#[cfg(test)]
#[test]
#[tracing_test::traced_test]
pub fn dir_check_test() {
    let tmat = test_matrix();

    assert_eq!(tmat.tree_visible_count(1, 2, Direction::Up), (1, 0, false));

    assert!(tmat.only_shorter(1, 1, Direction::Up));
    assert!(tmat.only_shorter(1, 1, Direction::Left));
    assert!(!tmat.only_shorter(1, 1, Direction::Down));
    assert!(!tmat.only_shorter(1, 1, Direction::Right));

    assert!(tmat.only_shorter(1, 2, Direction::Up));
    assert!(tmat.only_shorter(1, 2, Direction::Right));
    assert!(!tmat.only_shorter(1, 2, Direction::Left));
    assert!(!tmat.only_shorter(1, 2, Direction::Down));

    assert_eq!(tmat.tree_visible_count(3, 2, Direction::Up), (2, 1, true));
    assert_eq!(
        tmat.tree_visible_count(3, 2, Direction::Left),
        (2, 0, false)
    );
    assert_eq!(
        tmat.tree_visible_count(3, 2, Direction::Down),
        (1, 0, false)
    );
    assert_eq!(
        tmat.tree_visible_count(3, 2, Direction::Right),
        (2, 0, true)
    );

    assert_eq!(tmat.scenic_score(3, 2), 8);

    assert_eq!(tmat.tree_visible_count(2, 2, Direction::Up), (1, 1, true));
    assert_eq!(tmat.tree_visible_count(2, 2, Direction::Left), (1, 1, true));
    assert_eq!(tmat.tree_visible_count(2, 2, Direction::Down), (1, 1, true));
    assert_eq!(
        tmat.tree_visible_count(2, 2, Direction::Right),
        (1, 1, true)
    );

    assert_eq!(tmat.scenic_score(2, 2), 1);
}
