use crate::prelude::*;


pub(crate) fn run(mut buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();

    buffer.clear();

    let time_setup = start_setup.elapsed();

    let start_part_1 = Instant::now();
    let time_part_1 = start_part_1.elapsed();
    let start_part_2 = Instant::now();
    let time_part_2 = start_part_2.elapsed();

    Ok(RunData::new(
        0 as i64,
        0 as i64,
        Timing::new(
            time_setup,
            time_part_1,
            time_part_2,
            std::time::Duration::new(0, 0),
        ),
    ))
}

pub(crate) fn report(run_data: &RunData) -> crate::Result<()> {
    output::print_day(5, "Hydrothermal Venture")?;
    output::print_part(1, "🔥 Overlapping", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🔥 Overlapping", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
