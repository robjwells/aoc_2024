use foldhash::{HashMap, HashMapExt};

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let nums = parse(input);

    let p1 = part_one(&nums);
    assert_eq!(p1, 183_620, "Part one is incorrent.");
    let p2 = part_two(&nums);
    assert_eq!(p2, 220_377_651_399_268, "Part two is incorrent.");

    Answer::first(11, p1).second(p2).report()
}

fn part_one(nums: &[u64]) -> u64 {
    repeated_blink(25, nums)
}

fn part_two(nums: &[u64]) -> u64 {
    repeated_blink(75, nums)
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|part| part.parse().unwrap())
        .collect()
}

fn repeated_blink(blinks: u8, nums: &[u64]) -> u64 {
    fn update_counter(map: &mut HashMap<u64, u64>, key: u64, count: u64) {
        map.entry(key).and_modify(|v| *v += count).or_insert(count);
    }

    // Store the counts of each stone number, rather than having a Vec of
    // repeated stone numbers. After 25 blinks there are 432 different stone
    // numbers but 183,620 stones, and after 75 there are 3,777 numbers but
    // 220 *trillion* stones.
    let mut counter = HashMap::<u64, u64>::with_capacity(nums.len());
    for &n in nums {
        update_counter(&mut counter, n, 1);
    }

    for _ in 0..blinks {
        let mut working_counter = HashMap::<u64, u64>::with_capacity(counter.len());
        for (original_num, count) in counter {
            match blink(original_num) {
                (new_number, None) => {
                    update_counter(&mut working_counter, new_number, count);
                }
                (upper, Some(lower)) => {
                    update_counter(&mut working_counter, upper, count);
                    update_counter(&mut working_counter, lower, count);
                }
            }
        }
        counter = working_counter;
    }

    counter.values().sum()
}

fn blink(num: u64) -> (u64, Option<u64>) {
    if num == 0 {
        return (1, None);
    }

    let n_digits = num.ilog10() + 1;
    if n_digits % 2 == 0 {
        let divisor = 10_u64.pow(n_digits / 2);
        let upper = num / divisor;
        let lower = num % divisor;
        (upper, Some(lower))
    } else {
        (num * 2024, None)
    }
}

#[cfg(test)]
mod test {
    use super::{blink, parse, part_one};

    const SAMPLE_INPUT: &str = "0 1 10 99 999";
    const SAMPLE_NUMS: &[u64] = &[0, 1, 10, 99, 999];

    #[test]
    fn sample_input_single_blink() {
        assert_eq!(parse(SAMPLE_INPUT).as_slice(), SAMPLE_NUMS);
    }

    #[rstest::rstest]
    #[case(0, (1, None))]
    #[case(1, (2024, None))]
    #[case(10, (1, Some(0)))]
    #[case(99, (9, Some(9)))]
    #[case(999, (2_021_976, None))]
    pub fn test_blink(#[case] input: u64, #[case] expected: (u64, Option<u64>)) {
        assert_eq!(blink(input), expected);
    }

    #[test]
    pub fn sample_blink_25() {
        assert_eq!(part_one(&[125, 17]), 55_312);
    }
}
