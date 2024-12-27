#![allow(dead_code)]
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use glam::IVec2;
use itertools::Itertools;

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let grid: Grid = input.parse()?;
    let p1 = grid.count_antinode_positions();
    Answer::first(8, p1).report()
}

#[derive(Clone, Debug)]
struct Grid {
    height: usize,
    width: usize,
    antennas: HashMap<char, HashSet<Position>>,
}

impl Grid {
    fn antenna_positions(&self, antenna: char) -> Option<&HashSet<Position>> {
        self.antennas.get(&antenna)
    }

    fn in_bounds(&self, position: &Position) -> bool {
        let row = position.row() as usize;
        let col = position.col() as usize;
        (0..self.height).contains(&row) && (0..self.width).contains(&col)
    }

    fn antinode_positions_for_antenna(&self, antenna: char) -> Option<HashSet<Position>> {
        let antennas = self.antenna_positions(antenna)?;
        let antinodes = antennas
            .iter()
            .tuple_combinations()
            .flat_map(|(first, second)| {
                let diff = second - first;
                [first - diff, second + diff]
            })
            .filter(|position| self.in_bounds(position))
            .collect();
        Some(antinodes)
    }

    fn count_antinode_positions(&self) -> usize {
        self.antennas
            .keys()
            .filter_map(|&a| self.antinode_positions_for_antenna(a))
            .reduce(|mut acc, next| {
                acc.extend(next);
                acc
            })
            .map(|set| set.len())
            .unwrap_or_default()
    }
}

impl std::str::FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas: HashMap<char, HashSet<Position>> = HashMap::new();
        let mut height = 0;
        let mut width = 0;
        for (row_idx, row) in s.lines().enumerate() {
            height = row_idx + 1;
            for (col_idx, col) in row.chars().enumerate() {
                width = width.max(col_idx + 1);
                if col == '.' {
                    continue;
                }
                antennas
                    .entry(col)
                    .or_default()
                    .insert(Position::new(row_idx as i32, col_idx as i32));
            }
        }
        Ok(Self {
            height,
            width,
            antennas,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pos: IVec2,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self {
            pos: IVec2::new(row, col),
        }
    }

    fn row(&self) -> i32 {
        self.pos.x
    }

    fn col(&self) -> i32 {
        self.pos.y
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.row().cmp(&other.row()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.col().cmp(&other.col()),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position {{ row: {}, col: {} }}", self.row(), self.col())
    }
}

impl std::ops::Sub for &Position {
    type Output = IVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        self.pos - rhs.pos
    }
}

impl std::ops::Sub<IVec2> for &Position {
    type Output = Position;

    fn sub(self, rhs: IVec2) -> Self::Output {
        Position {
            pos: self.pos - rhs,
        }
    }
}

impl std::ops::Add<IVec2> for &Position {
    type Output = Position;

    fn add(self, rhs: IVec2) -> Self::Output {
        Position {
            pos: self.pos + rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use rstest::{fixture, rstest};

    use super::{Grid, Position};

    const SAMPLE_GRID_INPUT: &str = "\
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";

    const SAMPLE_INPUT_LARGE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[fixture]
    #[once]
    fn sample_large_grid() -> Grid {
        SAMPLE_INPUT_LARGE
            .parse()
            .expect("Failed to parse large sample grid.")
    }

    #[fixture]
    #[once]
    fn sample_grid() -> Grid {
        SAMPLE_GRID_INPUT
            .parse()
            .expect("Failed to parse sample grid.")
    }

    #[test]
    fn parse_input_grid() -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_GRID_INPUT.parse()?;
        let a_positions = grid.antenna_positions('a').unwrap();
        let expected = HashSet::from([Position::new(3, 4), Position::new(5, 5)]);
        assert_eq!(a_positions, &expected);
        assert_eq!((grid.height, grid.width), (10, 10));
        Ok(())
    }

    #[rstest]
    pub fn antinode_positions_for_a_part_one(sample_grid: &Grid) {
        let antinodes = sample_grid.antinode_positions_for_antenna('a').unwrap();
        let expected = HashSet::from([Position::new(1, 3), Position::new(7, 6)]);
        assert_eq!(antinodes, expected);
    }

    #[rstest]
    pub fn test_count_unique_antinode_positions_part_one(sample_large_grid: &Grid) {
        let answer = sample_large_grid.count_antinode_positions();
        assert_eq!(answer, 14);
    }
}
