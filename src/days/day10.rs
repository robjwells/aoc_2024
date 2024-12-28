use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let grid: Grid = input.parse()?;
    let guides = grid.produce_hiking_guides();

    let p1 = part_one(guides.as_slice());
    assert_eq!(p1, 501, "Part one is not correct.");

    let p2 = part_two(guides.as_slice());
    assert_eq!(p2, 1017, "Part two is not correct.");

    Answer::first(10, p1).second(p2).report()
}

fn part_one(guides: &[HikingGuide]) -> usize {
    guides.iter().map(|g| g.score()).sum()
}

fn part_two(guides: &[HikingGuide]) -> usize {
    guides.iter().map(|g| g.rating()).sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i8,
    col: i8,
}

#[derive(Debug, Clone, Copy)]
struct Trail {
    head: Position,
    current_position: Position,
    current_height: u8,
}

impl Trail {
    fn start_from(p: Position) -> Self {
        Self {
            head: p,
            current_position: p,
            current_height: 0,
        }
    }

    fn new_with_next(&self, pos: Position, height: u8) -> Trail {
        Self {
            head: self.head,
            current_position: pos,
            current_height: height,
        }
    }

    fn is_complete(&self) -> bool {
        self.current_height == 9
    }
}

#[derive(Debug)]
struct HikingGuide {
    #[allow(unused)]
    head: Position,
    unique_destinations: usize,
    distinct_trails: usize,
}

impl HikingGuide {
    fn score(&self) -> usize {
        self.unique_destinations
    }

    fn rating(&self) -> usize {
        self.distinct_trails
    }
}

#[derive(Debug)]
struct Grid {
    map: HashMap<Position, u8>,
    trail_starts: Vec<Trail>,
}

impl Grid {
    fn next_positions(&self, trail: Trail) -> Vec<(Position, u8)> {
        let Position { row, col } = trail.current_position;
        [
            Position { row: row - 1, col },
            Position { row: row + 1, col },
            Position { row, col: col - 1 },
            Position { row, col: col + 1 },
        ]
        .into_iter()
        .filter_map(|pos| {
            self.map
                .get(&pos)
                .filter(|&&height| height == trail.current_height + 1)
                .map(|height| (pos, *height))
        })
        .collect()
    }

    fn produce_hiking_guides(&self) -> Vec<HikingGuide> {
        let mut queue = VecDeque::from(self.trail_starts.clone());
        let mut complete: HashMap<Position, Vec<Trail>> = HashMap::with_capacity(queue.len());

        while let Some(trail) = queue.pop_front() {
            if trail.is_complete() {
                complete.entry(trail.head).or_default().push(trail);
                continue;
            }

            let new_trails = self
                .next_positions(trail)
                .into_iter()
                .map(|(pos, height)| trail.new_with_next(pos, height));
            queue.extend(new_trails);
        }

        complete
            .into_iter()
            .map(Grid::_produce_single_guide)
            .collect()
    }

    fn _produce_single_guide((head, trails): (Position, Vec<Trail>)) -> HikingGuide {
        let unique_destinations = trails
            .iter()
            .map(|t| t.current_position)
            .collect::<HashSet<_>>()
            .len();
        let distinct_trails = trails.len();
        HikingGuide {
            head,
            unique_destinations,
            distinct_trails,
        }
    }
}

impl std::str::FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut trail_starts = Vec::new();

        for (row_idx, row) in s.trim().lines().enumerate() {
            for (col_idx, col) in row.chars().enumerate() {
                let height = col.to_digit(10).expect("Non-digit in input.");
                let pos = Position {
                    row: row_idx as i8,
                    col: col_idx as i8,
                };
                map.insert(pos, height as u8);
                if height == 0 {
                    trail_starts.push(Trail::start_from(pos));
                }
            }
        }
        Ok(Self { map, trail_starts })
    }
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two, Grid, Position};

    const SAMPLE_TINY_GRID: &str = "\
0123
1234
8765
9876
";

    const SAMPLE_LARGE_GRID: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn parse_sample_input() -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_TINY_GRID.parse()?;
        assert_eq!(grid.map.get(&Position { row: 2, col: 2 }), Some(&6));
        Ok(())
    }

    #[test]
    fn sample_large_input_trailhead_score() -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_LARGE_GRID.parse()?;
        let guides = grid.produce_hiking_guides();
        let score = part_one(guides.as_slice());
        assert_eq!(score, 36);
        Ok(())
    }

    #[test]
    fn sample_large_input_trailhead_rating() -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_LARGE_GRID.parse()?;
        let guides = grid.produce_hiking_guides();
        let score = part_two(guides.as_slice());
        assert_eq!(score, 81);
        Ok(())
    }
}
