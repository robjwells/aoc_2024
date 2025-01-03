use anyhow::anyhow;

pub mod days;
mod util;

const PUZZLE_INPUT: &[&str] = &[
    include_str!("../input/2024-01.txt"),
    include_str!("../input/2024-02.txt"),
    include_str!("../input/2024-03.txt"),
    include_str!("../input/2024-04.txt"),
    include_str!("../input/2024-05.txt"),
    include_str!("../input/2024-06.txt"),
    include_str!("../input/2024-07.txt"),
    include_str!("../input/2024-08.txt"),
    include_str!("../input/2024-09.txt"),
    include_str!("../input/2024-10.txt"),
    include_str!("../input/2024-11.txt"),
];

type Solver = fn(&str) -> anyhow::Result<String>;

#[tracing::instrument]
pub fn run(day: usize) -> anyhow::Result<String> {
    assert_ne!(day, 0, "Day must be >= 1.");
    let days: &[Solver] = &[
        days::day01::solve,
        days::day02::solve,
        days::day03::solve,
        days::day04::solve,
        days::day05::solve,
        days::day06::solve,
        days::day07::solve,
        days::day08::solve,
        days::day09::solve,
        days::day10::solve,
        days::day11::solve,
    ];

    let Some(day_fn) = days.get(day - 1) else {
        return Err(anyhow!("Day {day} is not implemented yet."));
    };
    let Some(input) = PUZZLE_INPUT.get(day - 1) else {
        return Err(anyhow!("No input for day {day}."));
    };
    day_fn(input)
}
