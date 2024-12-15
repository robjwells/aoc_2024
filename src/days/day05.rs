use anyhow::anyhow;

use parse::{Rules, Updates};

pub fn solve(input: &str) -> anyhow::Result<String> {
    let (rules, updates) = parse_input(input)?;
    todo!();
}

fn parse_input(input: &str) -> anyhow::Result<(Rules, Updates)> {
    let (input, (rules, updates)) = parse::all(input).map_err(|e| e.to_owned())?;
    if !input.is_empty() {
        return Err(anyhow!("Leftover input: {input:?}"));
    }
    Ok((rules, updates))
}

mod parse {
    use std::collections::HashMap;

    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        multi::{fold_many1, many0, separated_list1},
        sequence::{separated_pair, terminated},
        IResult,
    };

    pub type Rules = HashMap<u32, Vec<u32>>;
    pub type Updates = Vec<Vec<u32>>;

    pub fn all(input: &str) -> IResult<&str, (Rules, Updates)> {
        let (input, rules) = rules(input)?;
        let (input, _) = line_ending(input)?;
        let (input, updates) = updates(input)?;
        let (input, _) = many0(line_ending)(input)?;
        IResult::Ok((input, (rules, updates)))
    }

    fn rules(input: &str) -> IResult<&str, Rules> {
        fold_many1(
            terminated(
                separated_pair(complete::u32, tag("|"), complete::u32),
                line_ending,
            ),
            HashMap::new,
            |mut rules: Rules, (before, after)| {
                rules.entry(before).or_default().push(after);
                rules
            },
        )(input)
    }

    fn updates(input: &str) -> IResult<&str, Updates> {
        separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)
    }
}

#[cfg(test)]
mod test {
    use rstest::{fixture, rstest};

    const SAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[fixture]
    #[once]
    fn setup_tracing() -> () {
        tracing_subscriber::fmt::init();
    }

    #[rstest]
    #[allow(unused_variables)]
    fn parse_sample_input(setup_tracing: &()) -> anyhow::Result<()> {
        let (rules, updates) = super::parse_input(SAMPLE_INPUT)?;

        let r97 = rules.get(&97).unwrap();
        assert_eq!(r97, &[13, 61, 47, 29, 53, 75]);

        let u = updates.first().unwrap();
        assert_eq!(u, &[75, 47, 61, 53, 29]);

        let u = updates.last().unwrap();
        assert_eq!(u, &[97, 13, 75, 29, 47]);

        Ok(())
    }
}
