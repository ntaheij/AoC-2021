use crate::prelude::*;

fn part_1(depths: &Vec<usize>) -> crate::Result<usize> {
    Ok(depths.windows(2).filter(|pair| pair[1] > pair[0]).count() as usize)
}

fn part_2(depths: &Vec<usize>) -> crate::Result<usize> {
    Ok(depths
        .windows(4)
        .filter(|quartet| quartet[3] > quartet[0])
        .count() as usize)
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();
    let depths: Vec<usize> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    let start_part_1 = Instant::now();
    let increases_1 = part_1(&depths)?;
    let time_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();
    let increases_2 = part_2(&depths)?;
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
    output::print_day(1, "Sonar Sweep")?;
    output::print_part(1, "📈 Increase", &format!("{}", run_data.part_1))?;
    output::print_part(2, "📈 Increase", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}