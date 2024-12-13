use anyhow::anyhow;
use tracing::instrument;

pub mod days;
mod util;

const PUZZLE_INPUT: &[&str] = &[
    include_str!("../input/2024-01.txt"),
    include_str!("../input/2024-02.txt"),
    include_str!("../input/2024-03.txt"),
];

type Solver = fn(&str) -> anyhow::Result<String>;

#[instrument]
pub fn run(day: usize) -> anyhow::Result<String> {
    assert_ne!(day, 0, "Day must be >= 1.");
    let days: &[Solver] = &[days::day01::solve, days::day02::solve, days::day03::solve];

    let Some(day_fn) = days.get(day - 1) else {
        return Err(anyhow!("Day {day} is not implemented yet."));
    };
    let Some(input) = PUZZLE_INPUT.get(day - 1) else {
        return Err(anyhow!("No input for day {day}."));
    };
    day_fn(input)
}
