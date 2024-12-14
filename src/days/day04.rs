use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

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
    grid.count_xmas_positions()
}

fn part_two(grid: &Grid) -> usize {
    grid.count_cross_mas_positions()
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Grid {
    // Using BTreeMaps/Sets over HashMaps/Sets as they're sorted by default,
    // which makes debugging printed output a little easier.
    map: BTreeMap<(usize, usize), char>,
    by_char: BTreeMap<char, BTreeSet<(usize, usize)>>,
}

impl Grid {
    fn at(&self, pos: &(usize, usize)) -> Option<&char> {
        self.map.get(pos)
    }

    fn check_char_at_position(&self, c: char, pos: &(usize, usize)) -> bool {
        self.at(pos).is_some_and(|&v| v == c)
    }

    fn deltas_to_valid_positions<const N: usize>(
        anchor: (usize, usize),
        deltas: [(isize, isize); N],
    ) -> Option<[(usize, usize); N]> {
        fn checked_tuple_add(
            (x_row, x_col): (usize, usize),
            (d_row, d_col): (isize, isize),
        ) -> Option<(usize, usize)> {
            let row = x_row.checked_add_signed(d_row)?;
            let col = x_col.checked_add_signed(d_col)?;
            Some((row, col))
        }

        let mapped = deltas.map(|d| checked_tuple_add(anchor, d));
        mapped
            .iter()
            .all(Option::is_some)
            .then(|| mapped.map(Option::unwrap))
    }

    fn count_xmas_from_x_position(&self, x_pos: (usize, usize)) -> usize {
        // Sanity check.
        assert!(
            self.check_char_at_position('X', &x_pos),
            "There is no X at {x_pos:?}"
        );
        // -mas deltas.
        [
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
        ]
        .into_iter()
        .filter_map(|deltas| Self::deltas_to_valid_positions(x_pos, deltas))
        .filter(|[m_pos, a_pos, s_pos]| {
            self.check_char_at_position('M', m_pos)
                && self.check_char_at_position('A', a_pos)
                && self.check_char_at_position('S', s_pos)
        })
        .count()
    }

    fn is_cross_mas_at_position(&self, a_pos: (usize, usize)) -> bool {
        // Sanity check.
        assert!(
            self.check_char_at_position('A', &a_pos),
            "There is no A at {a_pos:?}"
        );
        let deltas: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        let Some([top_left, top_right, bottom_left, bottom_right]) =
            Self::deltas_to_valid_positions(a_pos, deltas)
        else {
            return false;
        };

        let tl_br_mas = self.check_char_at_position('M', &top_left)
            && self.check_char_at_position('S', &bottom_right);
        let tl_br_sam = self.check_char_at_position('S', &top_left)
            && self.check_char_at_position('M', &bottom_right);

        let bl_tr_mas = self.check_char_at_position('M', &bottom_left)
            && self.check_char_at_position('S', &top_right);
        let bl_tr_sam = self.check_char_at_position('S', &bottom_left)
            && self.check_char_at_position('M', &top_right);

        (tl_br_mas || tl_br_sam) && (bl_tr_mas || bl_tr_sam)
    }

    fn count_xmas_positions(&self) -> usize {
        self.by_char
            .get(&'X')
            .unwrap()
            .iter()
            .map(|&x_pos| self.count_xmas_from_x_position(x_pos))
            .sum()
    }

    fn count_cross_mas_positions(&self) -> usize {
        self.by_char
            .get(&'A')
            .unwrap()
            .iter()
            .filter(|&&a_pos| self.is_cross_mas_at_position(a_pos))
            .count()
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

    #[rstest::fixture]
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

    #[test]
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
