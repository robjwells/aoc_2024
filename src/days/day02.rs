use crate::util::Answer;

pub fn solve(input: &str) -> String {
    let reports = parse_reports(input);
    let p1 = part_one(&reports);
    assert_eq!(p1, 257);
    let p2 = part_two(&reports);
    assert_eq!(p2, 328);
    Answer::first(2, p1).second(p2).to_string()
}

fn part_one(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| check_report(report).is_safe())
        .count()
}

fn part_two(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| check_report_dampened(report).is_safe())
        .count()
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    fn line_to_nums(line: &str) -> Vec<i32> {
        line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    }
    input.lines().map(line_to_nums).collect()
}

fn signs_match(a: i32, b: i32) -> bool {
    a.signum() == b.signum()
}

fn diff_in_range(level: i32) -> bool {
    matches!(level.abs(), 1..=3)
}

fn check_report(report: &[i32]) -> Check {
    let [first, second, rest @ ..] = report else {
        unreachable!("Reports are always longer than 2 levels.")
    };
    let mut current = *second;
    let mut last_diff = *first - *second;
    if !diff_in_range(last_diff) {
        return Check::Unsafe;
    }
    for next in rest.iter().copied() {
        let diff = current - next;
        if !(signs_match(diff, last_diff) && diff_in_range(diff)) {
            return Check::Unsafe;
        }
        current = next;
        last_diff = diff;
    }
    Check::Safe
}

fn check_report_dampened(report: &[i32]) -> Check {
    if check_report(report).is_safe() {
        Check::Safe
    } else {
        for idx in 0..report.len() {
            let mut new_report = report.to_vec();
            new_report.remove(idx);
            let check = check_report(&new_report);
            if check.is_safe() {
                return Check::Safe;
            }
        }
        Check::Unsafe
    }
}

#[derive(Debug)]
enum Check {
    Safe,
    Unsafe,
}

impl Check {
    /// Returns `true` if the check is [`Safe`].
    ///
    /// [`Safe`]: Check::Safe
    #[must_use]
    fn is_safe(&self) -> bool {
        matches!(self, Self::Safe)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::days::day02::check_report;

    const SAMPLE_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    fn sample_reports() -> Vec<Vec<i32>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn parse_sample_reports() {
        let reports = super::parse_reports(SAMPLE_INPUT);
        assert_eq!(reports, sample_reports())
    }

    #[rstest]
    #[case(&[7, 6, 4, 2, 1])]
    #[case(&[1, 3, 6, 7, 9])]
    #[trace]
    pub fn expected_safe_p1(#[case] report: &[i32]) {
        let check = super::check_report(report);
        assert!(check.is_safe());
    }

    #[rstest]
    #[case(&[1, 2, 7, 8, 9])]
    #[case(&[9, 7, 6, 2, 1])]
    #[case(&[1, 3, 2, 4, 5])]
    #[case(&[8, 6, 4, 4, 1])]
    #[trace]
    pub fn expected_unsafe_p1(#[case] report: &[i32]) {
        let check = super::check_report(report);
        assert!(!check.is_safe());
    }

    #[rstest]
    #[case(&[7, 6, 4, 2, 1])]
    #[case(&[1, 3, 6, 7, 9])]
    #[case(&[1, 3, 2, 4, 5])]
    #[case(&[8, 6, 4, 4, 1])]
    #[trace]
    pub fn expected_safe_p2(#[case] report: &[i32]) {
        let check = super::check_report_dampened(report);
        assert!(check.is_safe());
    }

    #[rstest]
    #[case(&[1, 2, 7, 8, 9])]
    #[case(&[9, 7, 6, 2, 1])]
    #[trace]
    pub fn expected_unsafe_p2(#[case] report: &[i32]) {
        let check = super::check_report_dampened(report);
        assert!(!check.is_safe());
    }

    #[test]
    pub fn solve_sample_part_one() {
        assert_eq!(super::part_one(&sample_reports()), 2);
    }

    #[test]
    pub fn solve_sample_part_two() {
        assert_eq!(super::part_two(&sample_reports()), 4);
    }

    #[test]
    pub fn ensure_first_two_levels_are_checked() {
        let report = &[14, 10, 9, 6, 4, 3, 2, 1];
        assert!(!check_report(report).is_safe());
    }
}
