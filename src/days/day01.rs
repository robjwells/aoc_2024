use counter::Counter;

use crate::util::Answer;

pub fn solve(input: &str) -> String {
    let (left, right) = parse_lists(input);
    let p1 = part_one(&left, &right);
    let p2 = part_two(&left, &right);
    Answer::first(1, p1).second(p2).to_string()
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

fn parse_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().map(|w| w.parse::<usize>().unwrap());
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            (left, right)
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
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
    fn parse_list_input_to_sorted() {
        let (left, right) = super::parse_lists(TEST_INPUT);
        assert_eq!(&left, &[1, 2, 3, 3, 3, 4], "Left column not correct.");
        assert_eq!(&right, &[3, 3, 3, 4, 5, 9], "Right column not correct.");
    }

    #[test]
    fn part_one_test_input() {
        let (left, right) = super::parse_lists(TEST_INPUT);
        assert_eq!(11, super::part_one(&left, &right));
    }

    #[test]
    fn part_two_test_input() {
        let (left, right) = super::parse_lists(TEST_INPUT);
        assert_eq!(31, super::part_two(&left, &right));
    }
}
