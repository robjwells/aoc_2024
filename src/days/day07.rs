use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::{many0, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use rayon::prelude::*;

use crate::util::Answer;

fn generate_operators(numbers_length: usize, operators: Vec<Symbol>) -> Vec<Vec<Symbol>> {
    let n_operators = numbers_length - 1;
    (0..n_operators)
        .map(|_| operators.clone())
        .multi_cartesian_product()
        .collect()
}

fn simple_operators() -> HashMap<usize, Vec<Vec<Symbol>>> {
    (2..=12)
        .map(|numbers_length| {
            (
                numbers_length,
                generate_operators(numbers_length, vec![Symbol::Add, Symbol::Multiply]),
            )
        })
        .collect()
}

fn complex_operators() -> HashMap<usize, Vec<Vec<Symbol>>> {
    (2..=12)
        .map(|numbers_length| {
            (
                numbers_length,
                generate_operators(
                    numbers_length,
                    vec![Symbol::Add, Symbol::Multiply, Symbol::Concat],
                ),
            )
        })
        .collect()
}

pub fn solve(input: &str) -> anyhow::Result<String> {
    let calibrations = parse_calibrations(input)?;

    let p1 = part_one(&calibrations);
    assert_eq!(p1, 2501605301465, "Part one is not correct.");

    let p2 = part_two(&calibrations);
    assert_eq!(p2, 44841372855953, "Part two is not correct.");

    Answer::first(7, p1).second(p2).report()
}

#[derive(Debug, Clone, Copy)]
enum Symbol {
    Add,
    Multiply,
    Concat,
}

fn calibrate_one_sequence(test_value: u64, nums: &[u64], symbols: &[Symbol]) -> bool {
    let mut nums = nums.iter();
    let mut total = *nums.next().unwrap();
    for (sym, rhs) in symbols.iter().zip(nums) {
        if total > test_value {
            return false;
        }
        match sym {
            Symbol::Add => total += *rhs,
            Symbol::Multiply => total *= *rhs,
            Symbol::Concat => {
                let shift_mul = 10_u64.pow(rhs.ilog10() + 1);
                total = total * shift_mul + rhs;
            }
        }
    }
    test_value == total
}

fn calibrate_all_sequences(
    calibrations: &[(u64, Vec<u64>)],
    operators: HashMap<usize, Vec<Vec<Symbol>>>,
) -> u64 {
    calibrations
        .par_iter()
        .filter(|(test_value, nums)| {
            operators[&nums.len()]
                .par_iter()
                .any(|ops| calibrate_one_sequence(*test_value, nums, ops))
        })
        .map(|(test_value, _)| *test_value)
        .sum()
}

fn part_one(calibrations: &[(u64, Vec<u64>)]) -> u64 {
    calibrate_all_sequences(calibrations, simple_operators())
}

fn part_two(calibrations: &[(u64, Vec<u64>)]) -> u64 {
    calibrate_all_sequences(calibrations, complex_operators())
}

type Calibrations = Vec<(u64, Vec<u64>)>;

fn parse_calibrations(input: &str) -> anyhow::Result<Calibrations> {
    let (input, calibrations) = terminated(
        separated_list1(line_ending, _parse_single_calibration),
        many0(line_ending),
    )(input)
    .map_err(|e| anyhow!("{e:?}"))?;
    assert!(input.is_empty(), "Leftover input: {input:?}");
    Ok(calibrations)
}

fn _parse_single_calibration(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(space1, complete::u64),
    )(input)
}

#[cfg(test)]
mod test {
    const SAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn parse_sample_calibrations() -> anyhow::Result<()> {
        let calibrations = super::parse_calibrations(SAMPLE_INPUT)?;
        assert_eq!(calibrations.first().unwrap(), &(190, vec![10, 19]));
        Ok(())
    }

    #[test]
    pub fn solve_sample_part_one() -> anyhow::Result<()> {
        let calibrations = super::parse_calibrations(SAMPLE_INPUT)?;
        let sum_of_valid_calibrations = super::part_one(&calibrations);
        assert_eq!(sum_of_valid_calibrations, 3749);
        Ok(())
    }

    #[test]
    pub fn solve_sample_part_two() -> anyhow::Result<()> {
        let calibrations = super::parse_calibrations(SAMPLE_INPUT)?;
        let sum_of_valid_calibrations = super::part_two(&calibrations);
        assert_eq!(sum_of_valid_calibrations, 11387);
        Ok(())
    }
}
