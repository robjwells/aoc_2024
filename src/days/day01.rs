use std::{num::ParseIntError, str::FromStr};

use counter::Counter;
use itertools::Itertools;

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let (left, right) = parse_lists(input)?;

    let p1 = part_one(&left, &right);
    assert_eq!(p1, 2378066); // Known-correct answer

    let p2 = part_two(&left, &right);
    assert_eq!(p2, 18934359); // Known-correct answer

    Answer::first(1, p1).second(p2).report()
}

fn part_one(left: &[usize], right: &[usize]) -> usize {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part_two(left: &[usize], right: &[usize]) -> usize {
    let counter: Counter<usize> = right.iter().copied().collect();
    left.iter().map(|n| n * counter[n]).sum()
}

fn parse_lists(input: &str) -> Result<(Vec<usize>, Vec<usize>), ParseIntError> {
    let nums: Vec<usize> = input
        .split_ascii_whitespace()
        .map(usize::from_str)
        .collect::<Result<Vec<usize>, ParseIntError>>()?;
    let (mut left, mut right): (Vec<_>, Vec<_>) = nums.into_iter().tuples().unzip();
    left.sort_unstable();
    right.sort_unstable();
    Ok((left, right))
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn parse_list_input_to_sorted() -> anyhow::Result<()> {
        let (left, right) = super::parse_lists(TEST_INPUT)?;
        assert_eq!(&left, &[1, 2, 3, 3, 3, 4], "Left column not correct.");
        assert_eq!(&right, &[3, 3, 3, 4, 5, 9], "Right column not correct.");
        Ok(())
    }

    #[test]
    fn part_one_test_input() -> anyhow::Result<()> {
        let (left, right) = super::parse_lists(TEST_INPUT)?;
        assert_eq!(11, super::part_one(&left, &right));
        Ok(())
    }

    #[test]
    fn part_two_test_input() -> anyhow::Result<()> {
        let (left, right) = super::parse_lists(TEST_INPUT)?;
        assert_eq!(31, super::part_two(&left, &right));
        Ok(())
    }
}
