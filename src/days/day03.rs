use regex::Regex;

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let p1 = part_one(input);
    assert_eq!(p1, 178794710, "Part one answer isn't correct");

    let p2 = part_two(input);
    assert_eq!(p2, 76729637, "Part two answer isn't correct");

    Answer::first(3, p1).second(p2).report()
}

fn part_one(input: &str) -> i32 {
    parse_instructions(input)
        .into_iter()
        .filter_map(|ins| match ins {
            Instruction::Multiply(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    use Instruction::{Do, Dont, Multiply};
    use State::{Disabled, Enabled};
    let (_, sum) = parse_instructions(input)
        .into_iter()
        .fold((Enabled, 0), |(state, sum), ins| match (state, ins) {
            (Enabled, Multiply(a, b)) => (state, sum + a * b),
            (Disabled, Multiply(_, _)) => (state, sum),
            (_, Do) => (Enabled, sum),
            (_, Dont) => (Disabled, sum),
        });
    sum
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Do,
    Dont,
    Multiply(i32, i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Enabled,
    Disabled,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mul_regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut instructions = Vec::new();
    for m in mul_regex.captures_iter(input) {
        let match_text = m.get(0).unwrap().as_str();
        if match_text.starts_with("don't") {
            instructions.push(Instruction::Dont);
        } else if match_text.starts_with("do") {
            instructions.push(Instruction::Do);
        } else if match_text.starts_with("mul") {
            let first: i32 = m.get(1).unwrap().as_str().parse().unwrap();
            let second: i32 = m.get(2).unwrap().as_str().parse().unwrap();
            instructions.push(Instruction::Multiply(first, second));
        } else {
            unreachable!("Only do, don't and mul are matched by the regex.")
        }
    }
    instructions
}

#[cfg(test)]
mod test {
    use super::Instruction;
    use rstest::*;

    const SAMPLE_INPUT_P1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_INSTRUCTIONS: &[Instruction] = &[
        Instruction::Multiply(2, 4),
        Instruction::Multiply(5, 5),
        Instruction::Multiply(11, 8),
        Instruction::Multiply(8, 5),
    ];
    const SAMPLE_INPUT_P2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const STRICT_SAMPLE_INSTRUCTIONS: &[Instruction] = &[
        Instruction::Multiply(2, 4),
        Instruction::Dont,
        Instruction::Multiply(5, 5),
        Instruction::Multiply(11, 8),
        Instruction::Do,
        Instruction::Multiply(8, 5),
    ];

    #[fixture]
    #[once]
    fn setup_tracing() -> () {
        tracing_subscriber::fmt::init();
    }

    #[test]
    fn parse_sample_input_p1() {
        assert_eq!(
            &super::parse_instructions(SAMPLE_INPUT_P1),
            SAMPLE_INSTRUCTIONS
        );
    }

    #[test]
    fn solve_sample_p1() {
        assert_eq!(super::part_one(SAMPLE_INPUT_P1), 161);
    }

    #[test]
    fn parse_sample_input_p2() {
        assert_eq!(
            &super::parse_instructions(SAMPLE_INPUT_P2),
            STRICT_SAMPLE_INSTRUCTIONS
        );
    }

    #[test]
    fn solve_sample_p2() {
        assert_eq!(super::part_two(SAMPLE_INPUT_P2), 48);
    }
}
