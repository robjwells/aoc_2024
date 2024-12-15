use std::cmp::Ordering;

use anyhow::anyhow;

use parse::{Rules, Updates};

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let (rules, updates) = parse_input(input)?;

    let p1 = part_one(&rules, &updates);
    assert_eq!(p1, 6612, "Part one answer is not correct.");

    let p2 = part_two(&rules, &updates);
    assert_eq!(p2, 4944, "Part two answer is not correct.");

    Answer::first(5, p1).second(p2).report()
}

fn part_one(rules: &Rules, updates: &Updates) -> u32 {
    updates
        .iter()
        .filter(|&update| update_is_valid(rules, update))
        .map(|xs| middle_element(xs))
        .sum()
}

fn part_two(rules: &Rules, updates: &Updates) -> u32 {
    updates
        .iter()
        .filter(|&update| !update_is_valid(rules, update))
        .map(|unsorted| sort_update(rules, unsorted))
        .map(|xs| middle_element(&xs))
        .sum()
}

fn sort_update(rules: &Rules, update: &[u32]) -> Vec<u32> {
    let mut update = update.to_vec();
    update.sort_by(|a, b| {
        let a_before_b = rules.get(a).is_some_and(|xs| xs.contains(b));
        let b_before_a = rules.get(b).is_some_and(|xs| xs.contains(a));

        if a_before_b {
            Ordering::Less
        } else if b_before_a {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    update
}

fn parse_input(input: &str) -> anyhow::Result<(Rules, Updates)> {
    let (input, (rules, updates)) = parse::all(input).map_err(|e| e.to_owned())?;
    if !input.is_empty() {
        return Err(anyhow!("Leftover input: {input:?}"));
    }
    Ok((rules, updates))
}

fn middle_element(xs: &[u32]) -> u32 {
    assert!(xs.len() & 1 == 1, "xs must be have an odd-numbered length.");
    xs[xs.len() / 2]
}

fn update_is_valid(rules: &Rules, update: &[u32]) -> bool {
    for (idx, appears_first) in update.iter().enumerate() {
        for appears_later in &update[idx + 1..] {
            let later_must_come_before_first = rules
                .get(appears_later)
                .is_some_and(|xs| xs.contains(appears_first));
            if later_must_come_before_first {
                return false;
            }
        }
    }
    true
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
    use rstest::fixture;

    use super::parse::Updates;

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

    #[test]
    fn parse_sample_input() -> anyhow::Result<()> {
        let (rules, updates) = super::parse_input(SAMPLE_INPUT)?;

        let r97 = rules.get(&97).unwrap();
        assert_eq!(r97, &[13, 61, 47, 29, 53, 75]);

        let u = updates.first().unwrap();
        assert_eq!(u, &[75, 47, 61, 53, 29]);

        let u = updates.last().unwrap();
        assert_eq!(u, &[97, 13, 75, 29, 47]);

        Ok(())
    }

    #[test]
    fn filter_sample_updates() -> anyhow::Result<()> {
        let (rules, updates) = super::parse_input(SAMPLE_INPUT)?;
        let ok: Updates = updates
            .into_iter()
            .filter(|u| super::update_is_valid(&rules, &u[..]))
            .collect();
        assert_eq!(
            ok,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
            ]
        );
        Ok(())
    }

    #[test]
    fn solve_sample_part_one() -> anyhow::Result<()> {
        let (rules, updates) = super::parse_input(SAMPLE_INPUT)?;
        let sum = super::part_one(&rules, &updates);
        assert_eq!(sum, 143);
        Ok(())
    }

    #[test]
    fn sort_update_correctly() -> anyhow::Result<()> {
        let (rules, _) = super::parse_input(SAMPLE_INPUT)?;
        let sorted = super::sort_update(&rules, &[61, 13, 29]);
        assert_eq!(sorted, &[61, 29, 13]);

        let sorted = super::sort_update(&rules, &[75, 97, 47, 61, 53]);
        assert_eq!(sorted, &[97, 75, 47, 61, 53]);

        let sorted = super::sort_update(&rules, &[97, 13, 75, 29, 47]);
        assert_eq!(sorted, &[97, 75, 47, 29, 13]);
        Ok(())
    }

    #[test]
    fn solve_sample_part_two() -> anyhow::Result<()> {
        let (rules, updates) = super::parse_input(SAMPLE_INPUT)?;
        let sum = super::part_two(&rules, &updates);
        assert_eq!(sum, 123);
        Ok(())
    }
}
