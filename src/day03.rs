use crate::prelude::*;

fn part_1(input: &[usize]) -> crate::Result<usize> {
    let x = (0..12).map(|i| max_bit(input, i) << i).sum::<usize>();

    // NOTE: Epsilon is gamma with flipped bits
    Ok(x * (!x & 0xfff))
}

fn part_2(input: &[usize]) -> crate::Result<usize> {
    Ok(_part_2(&input, 1) * _part_2(&input, 0)) 
}

fn _part_2(input: &[usize], oxygen: usize) -> usize {
    let mut input = input.to_vec();
    for i in (0..12).rev() {
      let keep = max_bit(&input, i) ^ oxygen;
      input.retain(|x| (x>>i) & 1 == keep);
      if input.len() == 1 { break }
    }
    input[0]
}

fn max_bit(nums: &[usize], bit: usize) -> usize {
    let mut c = [0,0];
    for &x in nums {
      c[(x as usize >> bit) & 1] += 1
    }
    (c[1] >= c[0]) as usize
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();
    let input = buffer.lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
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
    output::print_day(3, "Binary Diagnostic")?;
    output::print_part(1, "🖥️ Power Consumption", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🖥️ Life Support Rating", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
