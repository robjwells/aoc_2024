use anyhow::anyhow;
use tracing::instrument;

pub mod days;
mod util;

const PUZZLE_INPUT: &[&str] = &[
    include_str!("../input/2024-01.txt"),
    include_str!("../input/2024-02.txt"),
];

#[instrument]
pub fn run(day: usize) -> anyhow::Result<String> {
    assert_ne!(day, 0, "Day must be >= 1.");
    let days: &[fn(&str) -> String] = &[days::day01::solve, days::day02::solve];

    let Some(day_fn) = days.get(day - 1) else {
        return Err(anyhow!("Day {day} is not implemented yet."));
    };
    let Some(input) = PUZZLE_INPUT.get(day - 1) else {
        return Err(anyhow!("No input for day {day}."));
    };

    Ok(day_fn(input))
}
