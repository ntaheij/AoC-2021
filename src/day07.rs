use crate::prelude::*;
fn gauss_sum(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn part_1(positions: &Vec<u16>) -> crate::Result<u32> {
    let median = positions[positions.len() / 2];
    let fuel = positions
        .iter()
        .map(|position| (*position as i32 - median as i32).abs() as u32)
        .sum();
    Ok(fuel)
}

fn part_2(positions: &Vec<u16>) -> crate::Result<u32> {
    let average_float =
        positions.iter().map(|n| *n as u32).sum::<u32>() as f32 / positions.len() as f32;
    let average_floor = average_float.floor() as u32;
    let average_ceil = average_float.ceil() as u32;
    let (fuel_floor, fuel_ceil) = positions.iter().fold((0_u32, 0_u32), |acc, position| {
        let distance_floor = (*position as i32 - average_floor as i32).abs() as u32;
        let distance_ceil = (*position as i32 - average_ceil as i32).abs() as u32;
        (
            acc.0 + gauss_sum(distance_floor),
            acc.1 + gauss_sum(distance_ceil),
        )
    });
    Ok(std::cmp::min(fuel_floor, fuel_ceil))
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();
    let input: Vec<u16> = buffer
        .trim()
        .split(',')
        .map(|position| position.parse().expect("failed to parse position"))
        .collect();
    let time_setup = start_setup.elapsed();

    let start_part_1 = Instant::now();
    let increases_1 = part_1(&input)?;
    let time_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();
    let increases_2 = part_2(&input)?;
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
