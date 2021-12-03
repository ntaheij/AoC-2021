use crate::prelude::*;

fn part_1(input: &Vec<String>) -> crate::Result<i64> {
    let num_bits = input[0].len();
    assert!(input.iter().all(|x| x.len() == num_bits));

    let mut freq = vec![0; num_bits];
    for diag in input.iter() {
        for i in 0..num_bits {
            if diag.bytes().nth(i) == Some(b'1') {
                freq[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    for f in freq.iter() {
        gamma *= 2;
        if f + f >= input.len() {
            gamma += 1;
        }
    }

    // Epsilon is gama with flipped bits
    let epsilon = ((1 << num_bits ) - 1) ^ gamma;

    Ok(gamma * epsilon)
}

fn _part_2(input: &Vec<String>, idx: usize, win: u8) -> i64 {
    if input.len() == 1 {
        return i64::from_str_radix(&input[0], 2).unwrap();
    }

    let num_bits = input[0].len();
    assert!(input.iter().all(|x| x.len() == num_bits));

    let f: usize = input
        .iter()
        .map(|s| {
            if s.bytes().nth(idx) == Some(b'1') {
                1
            } else {
                0
            }
        })
        .sum();
    let goal = if f + f >= input.len() { win } else { win ^ 1 };

    let sub_inputs = &input
        .into_iter()
        .filter(|s| s.bytes().nth(idx) == Some(goal))
        .cloned()
        .collect();

    _part_2(&sub_inputs, idx + 1, win)
}

fn part_2(input: &Vec<String>) -> crate::Result<i64> {
    let oxygen =  _part_2(input, 0, b'1');
    let co2 =  _part_2(input, 0, b'0');

    Ok(oxygen * co2) 
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
    output::print_day(2, "Binary Diagnostic")?;
    output::print_part(1, "🖥️ Power Consumption", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🖥️ Life Support Rating", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
