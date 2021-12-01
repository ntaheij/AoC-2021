use crate::prelude::*;

fn part_1(depths: &Vec<i32>) -> crate::Result<i32> {
    Ok(depths.windows(2).filter(|pair| pair[1] > pair[0]).count() as i32)
}

fn part_2(depths: &Vec<i32>) -> crate::Result<i32> {
    Ok(depths
        .windows(4)
        .filter(|quartet| quartet[3] > quartet[0])
        .count() as i32)
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    // Read values from input
    let start_setup = Instant::now();
    let depths: Vec<i32> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // Look for increases
    let start_part_1 = Instant::now();
    let increases_1 = part_1(&depths)?;
    let time_part_1 = start_part_1.elapsed();

    // Look for increases in window of 3 values
    let start_part_2 = Instant::now();
    let increases_2 = part_2(&depths)?;
    let time_part_2 = start_part_2.elapsed();

    // Return
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
    output::print_day(1, "Sonar Sweep")?;
    output::print_part(1, "📈 Increase", &format!("{}", run_data.part_1))?;
    output::print_part(2, "📈 Increase", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}