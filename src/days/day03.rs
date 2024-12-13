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
        .map(Multiply::mul)
        .sum()
}

fn part_two(input: &str) -> i32 {
    strict_parse_instructions(input)
        .into_iter()
        .map(Multiply::mul)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Multiply(i32, i32);

impl Multiply {
    fn mul(self) -> i32 {
        self.0 * self.1
    }
}

fn parse_instructions(input: &str) -> Vec<Multiply> {
    let mul_regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut instructions = Vec::new();
    for m in mul_regex.captures_iter(input) {
        let first: i32 = m.get(1).unwrap().as_str().parse().unwrap();
        let second: i32 = m.get(2).unwrap().as_str().parse().unwrap();
        instructions.push(Multiply(first, second));
    }
    instructions
}

fn strict_parse_instructions(input: &str) -> Vec<Multiply> {
    let mul_regex = regex::Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut instructions = Vec::new();
    let mut enabled = true;
    for m in mul_regex.captures_iter(input) {
        let text = m.get(0).unwrap().as_str();
        if text.starts_with("don't") {
            enabled = false;
        } else if text.starts_with("do") {
            enabled = true;
        } else if text.starts_with("mul") && enabled {
            let first: i32 = m.get(1).unwrap().as_str().parse().unwrap();
            let second: i32 = m.get(2).unwrap().as_str().parse().unwrap();
            instructions.push(Multiply(first, second));
        }
    }
    instructions
}

#[cfg(test)]
mod test {
    use super::Multiply;
    use rstest::*;

    const SAMPLE_INPUT_P1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_INSTRUCTIONS: &[Multiply] = &[
        Multiply(2, 4),
        Multiply(5, 5),
        Multiply(11, 8),
        Multiply(8, 5),
    ];
    const SAMPLE_INPUT_P2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const STRICT_SAMPLE_INSTRUCTIONS: &[Multiply] = &[Multiply(2, 4), Multiply(8, 5)];

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
            &super::strict_parse_instructions(SAMPLE_INPUT_P2),
            STRICT_SAMPLE_INSTRUCTIONS
        );
    }

    #[test]
    fn solve_sample_p2() {
        assert_eq!(super::part_two(SAMPLE_INPUT_P2), 48);
    }
}
