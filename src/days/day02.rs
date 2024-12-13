use std::num::ParseIntError;

use crate::util::Answer;

use itertools::Itertools;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let reports = parse_reports(input)?;

    let p1 = part_one(&reports);
    assert_eq!(p1, 257); // Known-correct answer

    let p2 = part_two(&reports);
    assert_eq!(p2, 328); // Known-correct answer

    Answer::first(2, p1).second(p2).report()
}

fn part_one(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| report_is_safe(report))
        .count()
}

fn part_two(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| dampened_report_is_safe(report))
        .count()
}

fn parse_reports(input: &str) -> Result<Vec<Vec<i32>>, ParseIntError> {
    fn line_to_nums(line: &str) -> Result<Vec<i32>, ParseIntError> {
        line.split_whitespace().map(|n| n.parse()).collect()
    }
    input.lines().map(line_to_nums).collect()
}

fn report_is_safe(report: &[i32]) -> bool {
    fn signs_match((a, b): (&i32, &i32)) -> bool {
        a.signum() == b.signum()
    }

    fn diff_in_range(level: &i32) -> bool {
        matches!(level.abs(), 1..=3)
    }

    let diffs: Vec<i32> = report
        .iter()
        .tuple_windows()
        .map(|(first, second)| first - second)
        .collect();
    let all_diffs_in_range = diffs.iter().all(diff_in_range);
    let all_diffs_have_same_sign = diffs.iter().tuple_windows().all(signs_match);

    all_diffs_in_range && all_diffs_have_same_sign
}

fn dampened_report_is_safe(report: &[i32]) -> bool {
    if report_is_safe(report) {
        true
    } else {
        for idx in 0..report.len() {
            let mut new_report = report.to_vec();
            new_report.remove(idx);
            if report_is_safe(&new_report) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use std::num::ParseIntError;

    use rstest::rstest;

    use crate::days::day02::report_is_safe;

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
    fn parse_sample_reports() -> Result<(), ParseIntError> {
        let reports = super::parse_reports(SAMPLE_INPUT)?;
        assert_eq!(reports, sample_reports());
        Ok(())
    }

    #[rstest]
    #[case(&[7, 6, 4, 2, 1])]
    #[case(&[1, 3, 6, 7, 9])]
    #[trace]
    pub fn expected_safe_p1(#[case] report: &[i32]) {
        assert!(super::report_is_safe(report));
    }

    #[rstest]
    #[case(&[1, 2, 7, 8, 9])]
    #[case(&[9, 7, 6, 2, 1])]
    #[case(&[1, 3, 2, 4, 5])]
    #[case(&[8, 6, 4, 4, 1])]
    #[trace]
    pub fn expected_unsafe_p1(#[case] report: &[i32]) {
        assert!(!super::report_is_safe(report));
    }

    #[rstest]
    #[case(&[7, 6, 4, 2, 1])]
    #[case(&[1, 3, 6, 7, 9])]
    #[case(&[1, 3, 2, 4, 5])]
    #[case(&[8, 6, 4, 4, 1])]
    #[trace]
    pub fn expected_safe_p2(#[case] report: &[i32]) {
        assert!(super::dampened_report_is_safe(report));
    }

    #[rstest]
    #[case(&[1, 2, 7, 8, 9])]
    #[case(&[9, 7, 6, 2, 1])]
    #[trace]
    pub fn expected_unsafe_p2(#[case] report: &[i32]) {
        assert!(!super::dampened_report_is_safe(report));
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
        assert!(!report_is_safe(report));
    }
}
