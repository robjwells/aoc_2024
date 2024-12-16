use std::collections::BTreeSet;
use std::str::FromStr;

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let grid = input.parse()?;

    let p1 = count_visited_positions(&grid);
    assert_eq!(p1, 4722, "Part one is not the correct answer.");

    Answer::first(6, p1).report()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

struct Grid {
    walls: BTreeSet<Position>,
    start_position: Position,
    n_rows: usize,
    n_cols: usize,
}

impl Grid {
    fn start_position(&self) -> (Position, Direction) {
        (self.start_position, Direction::North)
    }

    fn in_bounds(&self, position: Position) -> bool {
        position.row < self.n_rows && position.col < self.n_cols
    }

    fn next_position(
        &self,
        position: Position,
        direction: Direction,
    ) -> Option<(Position, Direction)> {
        let Position { row, col } = position;
        let next_pos = match direction {
            Direction::North => (row.checked_sub(1), Some(col)),
            Direction::East => (Some(row), Some(col + 1)),
            Direction::South => (Some(row + 1), Some(col)),
            Direction::West => (Some(row), col.checked_sub(1)),
        };
        let (Some(next_row), Some(next_col)) = next_pos else {
            return None;
        };
        let next_pos = Position::new(next_row, next_col);
        if !self.in_bounds(next_pos) {
            return None;
        }
        if self.walls.contains(&next_pos) {
            return self.next_position(position, direction.turn_right());
        }
        Some((next_pos, direction))
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<Position> = None;
        let mut walls: BTreeSet<Position> = BTreeSet::new();

        let n_rows = s.lines().count();
        let n_cols = s
            .lines()
            .next()
            .map(|l| l.len())
            .expect("No lines in grid.");

        for (row_idx, row) in s.lines().enumerate() {
            for (col_idx, element) in row.chars().enumerate() {
                if element == '#' {
                    walls.insert(Position::new(row_idx, col_idx));
                } else if element == '^' {
                    start = Some(Position::new(row_idx, col_idx));
                }
            }
        }
        Ok(Self {
            walls,
            start_position: start.expect("No start position found."),
            n_rows,
            n_cols,
        })
    }
}

fn count_visited_positions(grid: &Grid) -> usize {
    let (mut pos, mut dir) = grid.start_position();
    let mut visited = BTreeSet::from([pos]);
    while let Some(next) = grid.next_position(pos, dir) {
        (pos, dir) = next;
        visited.insert(pos);
    }
    visited.len()
}

#[cfg(test)]
mod test {
    use super::{count_visited_positions, Direction, Grid, Position};
    use rstest::{fixture, rstest};

    const SAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[fixture]
    #[once]
    fn setup_tracing() -> () {
        tracing_subscriber::fmt::init();
    }

    #[test]
    fn parse_sample_grid() -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_INPUT.parse()?;
        assert_eq!(
            grid.start_position(),
            (Position::new(6, 4), Direction::North)
        );
        assert!(!grid.in_bounds(Position::new(10, 0)));
        assert!(!grid.in_bounds(Position::new(0, 10)));
        Ok(())
    }

    #[rstest]
    #[allow(unused_variables)]
    fn trace_sample_grid(setup_tracing: &()) -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_INPUT.parse()?;
        let n_visited = count_visited_positions(&grid);
        assert_eq!(n_visited, 41);
        Ok(())
    }
}
