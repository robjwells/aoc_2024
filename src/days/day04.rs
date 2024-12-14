#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use tracing::instrument;

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let grid: Grid = input.parse()?;

    let p1 = part_one(&grid);
    assert_eq!(p1, 2447, "Part one isn't the correct answer.");

    let p2 = part_two(&grid);
    assert_eq!(p2, 1868, "Part two isn't the correct answer.");

    Answer::first(4, p1).second(p2).report()
}

fn part_one(grid: &Grid) -> usize {
    grid.find_xmas_positions().len()
}

fn part_two(grid: &Grid) -> usize {
    grid.find_cross_mas_positions().len()
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Grid {
    map: BTreeMap<(usize, usize), char>,
    by_char: BTreeMap<char, BTreeSet<(usize, usize)>>,
}

impl Grid {
    #[allow(unused)]
    fn at(&self, pos: &(usize, usize)) -> Option<&char> {
        self.map.get(pos)
    }

    const XMAS_DELTAS: [[(isize, isize); 3]; 8] = [
        // Up and left.
        [(-1, -1), (-2, -2), (-3, -3)],
        // Up.
        [(-1, 0), (-2, 0), (-3, 0)],
        // Up and right.
        [(-1, 1), (-2, 2), (-3, 3)],
        // Right.
        [(0, 1), (0, 2), (0, 3)],
        // Down and right.
        [(1, 1), (2, 2), (3, 3)],
        // Down.
        [(1, 0), (2, 0), (3, 0)],
        // Down and left.
        [(1, -1), (2, -2), (3, -3)],
        // Left.
        [(0, -1), (0, -2), (0, -3)],
    ];

    fn checked_tuple_add(
        (x_row, x_col): (usize, usize),
        (d_row, d_col): (isize, isize),
    ) -> Option<(usize, usize)> {
        let row = x_row.checked_add_signed(d_row)?;
        let col = x_col.checked_add_signed(d_col)?;
        Some((row, col))
    }

    fn deltas_to_valid_positions(
        x: (usize, usize),
        deltas: [(isize, isize); 3],
    ) -> Option<[(usize, usize); 3]> {
        let [md, ad, sd] = deltas;
        let m = Self::checked_tuple_add(x, md)?;
        let a = Self::checked_tuple_add(x, ad)?;
        let s = Self::checked_tuple_add(x, sd)?;
        Some([m, a, s])
    }

    fn char_at_position(&self, c: char, pos: &(usize, usize)) -> bool {
        self.at(pos).is_some_and(|v| v == &c)
    }

    #[instrument(skip(self))]
    fn valid_projections_from_x(&self, x_pos: (usize, usize)) -> Vec<[(usize, usize); 3]> {
        Self::XMAS_DELTAS
            .into_iter()
            .filter_map(|deltas| Self::deltas_to_valid_positions(x_pos, deltas))
            .filter(|[m_pos, a_pos, s_pos]| {
                self.char_at_position('M', m_pos)
                    && self.char_at_position('A', a_pos)
                    && self.char_at_position('S', s_pos)
            })
            .collect()
    }

    #[instrument(skip(self))]
    fn cross_mas_at_position(&self, a_pos: (usize, usize)) -> bool {
        let deltas: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        let mapped = deltas.map(|d| Self::checked_tuple_add(a_pos, d));
        let [Some(tl), Some(tr), Some(bl), Some(br)] = mapped else {
            // Not all deltas are at valid positions.
            return false;
        };

        let tl_br_in_order = self.char_at_position('M', &tl) && self.char_at_position('S', &br);
        let tl_br_reversed = self.char_at_position('S', &tl) && self.char_at_position('M', &br);

        let bl_tr_in_order = self.char_at_position('M', &bl) && self.char_at_position('S', &tr);
        let bl_tr_reversed = self.char_at_position('S', &bl) && self.char_at_position('M', &tr);

        (tl_br_in_order || tl_br_reversed) && (bl_tr_in_order || bl_tr_reversed)
    }

    #[instrument(skip(self))]
    fn find_xmas_positions(&self) -> Vec<[(usize, usize); 4]> {
        self.by_char
            .get(&'X')
            .unwrap()
            .iter()
            .flat_map(|&x| {
                self.valid_projections_from_x(x)
                    .into_iter()
                    .map(move |[m, a, s]| [x, m, a, s])
            })
            .collect()
    }

    fn find_cross_mas_positions(&self) -> Vec<(usize, usize)> {
        self.by_char
            .get(&'A')
            .unwrap()
            .iter()
            .filter(|&&a_pos| self.cross_mas_at_position(a_pos))
            .copied()
            .collect()
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Grid = Default::default();

        for (row_idx, line) in s.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let position = (row_idx, col_idx);
                grid.map.insert(position, c);
                grid.by_char.entry(c).or_default().insert(position);
            }
        }

        Ok(grid)
    }
}

#[cfg(test)]
mod test {
    use super::Grid;
    use rstest::{fixture, rstest};

    const SAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[fixture]
    #[once]
    fn setup_tracing() -> () {
        tracing_subscriber::fmt::init();
    }

    #[test]
    fn parse_sample_input() -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_INPUT.parse()?;
        assert_eq!(grid.at(&(0, 0)), Some(&'M'));
        assert_eq!(grid.at(&(9, 9)), Some(&'X'));
        Ok(())
    }

    #[rstest]
    fn solve_sample_part_one() -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_INPUT.parse()?;
        assert_eq!(super::part_one(&grid), 18);
        Ok(())
    }

    #[test]
    fn solve_sample_part_two() -> anyhow::Result<()> {
        let grid: Grid = SAMPLE_INPUT.parse()?;
        assert_eq!(super::part_two(&grid), 9);
        Ok(())
    }
}
