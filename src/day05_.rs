use crate::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;

#[warn(dead_code)]
type Parsed = Vec<((i64, i64), (i64, i64))>;

#[warn(dead_code)]
fn part_1(input: &Parsed) -> usize {
    let mut graph: HashMap<(i64, i64), u64> = HashMap::new();
    input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .for_each(|((x1, y1), (x2, y2))| {
            let mut x = *x1;
            let mut y = *y1;
            loop {
                *graph.entry((x, y)).or_insert(0) += 1;

                if y == *y2 && x == *x2 {
                    break;
                }
                match y.cmp(y2) {
                    Ordering::Greater => y -= 1,
                    Ordering::Less => y += 1,
                    Ordering::Equal => {}
                }
                match x.cmp(x2) {
                    Ordering::Greater => x -= 1,
                    Ordering::Less => x += 1,
                    Ordering::Equal => {}
                }
            }
        });

    graph.values().filter(|v| **v > 1).count()
}

#[warn(dead_code)]
fn part_2(input: &Parsed) -> usize {
    let mut graph: HashMap<(i64, i64), u64> = HashMap::new();

    input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2 || (y2 - y1).abs() == (x2 - x1).abs())
        .for_each(|((x1, y1), (x2, y2))| {
            let mut x = *x1;
            let mut y = *y1;
            loop {
                *graph.entry((x, y)).or_insert(0) += 1;

                if y == *y2 && x == *x2 {
                    break;
                }
                match y.cmp(y2) {
                    Ordering::Greater => y -= 1,
                    Ordering::Less => y += 1,
                    Ordering::Equal => {}
                }
                match x.cmp(x2) {
                    Ordering::Greater => x -= 1,
                    Ordering::Less => x += 1,
                    Ordering::Equal => {}
                }
            }
        });

    graph.values().filter(|v| **v > 1).count()
}

#[warn(unused_variables)]
pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();
    let input = buffer
        .lines()
        .map(|l| {
            let mut split = l.split(" -> ");
            let mut start = split.next().unwrap().split(',');
            let mut stop = split.next().unwrap().split(',');
            (
                (
                    start.next().unwrap().parse().unwrap(),
                    start.next().unwrap().parse().unwrap(),
                ),
                (
                    stop.next().unwrap().parse().unwrap(),
                    stop.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect();
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
    output::print_day(5, "Hydrothermal Venture")?;
    output::print_part(1, "🔥 Overlapping", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🔥 Overlapping", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
