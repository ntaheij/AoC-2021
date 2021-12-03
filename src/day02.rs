use crate::prelude::*;

fn part_1(input: &Vec<String>) -> crate::Result<i64> {
    let mut x = 0;
    let mut depth = 0;

    for line in input {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        let arg = split.next().unwrap().parse::<i64>().unwrap();

        match command {
            "forward" => x += arg,
            "down" => depth += arg,
            "up" => depth -= arg,
            _ => panic!(),
        }
    }

    Ok(x * depth)
}

fn part_2(input: &Vec<String>) -> crate::Result<i64> {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        let arg = split.next().unwrap().parse::<i64>().unwrap();

        match command {
            "forward" => {
                x += arg;
                depth += aim * arg;
            }
            "down" => aim += arg,
            "up" => aim -= arg,
            _ => panic!(),
        }
    }

    Ok(x * depth) 
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    // Read values from input
    let start_setup = Instant::now();
    let input: Vec<String> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // Look for increases
    let start_part_1 = Instant::now();
    let increases_1 = part_1(&input)?;
    let time_part_1 = start_part_1.elapsed();

    // Look for increases in window of 3 values
    let start_part_2 = Instant::now();
    let increases_2 = part_2(&input)?;
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
    output::print_day(2, "Dive!")?;
    output::print_part(1, "📉 DepthHeight", &format!("{}", run_data.part_1))?;
    output::print_part(2, "📉 DepthHeight", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
