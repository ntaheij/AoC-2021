use crate::prelude::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Brace {
    Angle,
    Curly,
    Paren,
    Square,
}

impl Brace {
    fn new(c: char) -> (Self, bool) {
        match c {
            '<' => (Self::Angle, false),
            '{' => (Self::Curly, false),
            '(' => (Self::Paren, false),
            '[' => (Self::Square, false),
            '>' => (Self::Angle, true),
            '}' => (Self::Curly, true),
            ')' => (Self::Paren, true),
            ']' => (Self::Square, true),
            c => panic!("{}", c),
        }
    }
}

fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for (brace, close) in line.chars().map(Brace::new) {
                if close {
                    if Some(brace) != stack.pop() {
                        return match brace {
                            Brace::Angle => 25137,
                            Brace::Curly => 1197,
                            Brace::Paren => 3,
                            Brace::Square => 57,
                        };
                    }
                } else {
                    stack.push(brace);
                }
            }
            0
        })
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut output: Vec<_> = input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for (brace, close) in line.chars().map(Brace::new) {
                if close {
                    if Some(brace) != stack.pop() {
                        return 0;
                    }
                } else {
                    stack.push(brace);
                }
            }
            stack.into_iter().rev().fold(0, |acc, brace| {
                acc * 5
                    + match brace {
                        Brace::Angle => 4,
                        Brace::Curly => 3,
                        Brace::Paren => 1,
                        Brace::Square => 2,
                    }
            })
        })
        .filter(|x| *x != 0)
        .collect();
    output.sort();
    output[output.len() / 2]
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();
    let input = buffer.to_string();
    let time_setup = start_setup.elapsed();

    let start_part_1 = Instant::now();
    let increases_1 = part_1(&input);
    let time_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();
    let increases_2 = part_2(&input);
    let time_part_2 = start_part_2.elapsed();

    Ok(RunData::new(
        increases_1 as i64,
        increases_2 as i64,
        Timing::new(
            time_setup,
            time_part_1,
            time_part_2,
            std::time::Duration::new(0, 0),
        ),
    ))
}

pub(crate) fn report(run_data: &RunData) -> crate::Result<()> {
    output::print_day(19, "Syntax Scoring")?;
    output::print_part(1, "🚬 Syntax Score", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🚬 Middle Score", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
