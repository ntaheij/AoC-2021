use crate::prelude::*;

pub fn part_1(input: Vec<u8>) -> usize {
    simulate_fish(input, 80)
}
pub fn part_2(input: Vec<u8>) -> usize {
    simulate_fish(input, 256)
}

fn simulate_fish(lanternfish: Vec<u8>, days: usize) -> usize {
    let mut counts = [0; 9];
    for fish in lanternfish {
        counts[fish as usize] += 1;
    }

    for _day in 0..days {
        let new_fish = counts[0];
        for left in 0..counts.len() - 1 {
            counts[left] = counts[left + 1];
        }
        counts[8] = new_fish;
        counts[6] += new_fish;
    }

    counts.iter().sum()
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();
    let input = buffer
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
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
    output::print_day(6, "Lanternfish")?;
    output::print_part(1, "🔥 Overlapping", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🔥 Overlapping", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
