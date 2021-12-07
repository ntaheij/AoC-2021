use crate::prelude::*;
use itertools::Itertools;

pub fn part_2(input: Vec<i32>) -> i32 {
    let (min, max) = input.iter()
        .minmax()
        .into_option().unwrap();

    (*min..=*max).into_iter()
        .map(|i| {
            input.iter()
                .map(|crab| (*crab - i).abs())
                .sum()
        })
        .min().unwrap()
}

pub fn part_1(input: Vec<i32>) -> i32 {
    let (min, max) = input.iter()
        .minmax()
        .into_option().unwrap();

    (*min..=*max).into_iter()
        .map(|i| {
            input.iter()
                .map(|crab| (*crab - i).abs())
                .map(|n| n * (n + 1) / 2)
                .sum()
        })
        .min().unwrap()
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();
    let input = buffer.split(',').into_iter()
        .map(|i| i.parse::<i32>().unwrap())
        .collect_vec();
    let time_setup = start_setup.elapsed();

    let start_part_1 = Instant::now();
    let increases_1 = part_1(input.clone());
    let time_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();
    let increases_2 = part_2(input.clone());
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
    output::print_day(7, "The Treachery of Whales")?;
    output::print_part(1, "🐋 Fuel Spent", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🐋 Fuel Spent", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
