use std::fmt::Display;

pub struct Answer {
    day: usize,
    first: String,
    second: String,
}
pub struct PartialAnswer {
    day: usize,
    first: String,
}

impl Answer {
    pub fn first<T: Display>(day: usize, answer: T) -> PartialAnswer {
        PartialAnswer {
            day,
            first: answer.to_string(),
        }
    }
}

impl PartialAnswer {
    pub fn second<T: Display>(self, answer: T) -> Answer {
        Answer {
            day: self.day,
            first: self.first,
            second: answer.to_string(),
        }
    }
}

impl Display for PartialAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Day {}", self.day)?;
        writeln!(f, "========================")?;
        writeln!(f, "Part one: {:>14}", self.first)
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Day {}", self.day)?;
        writeln!(f, "========================")?;
        writeln!(f, "Part one: {:>14}", self.first)?;
        writeln!(f, "Part two: {:>14}", self.second)
    }
}
