use crate::prelude::*;
use std::collections::HashSet;

type Board = Vec<Vec<usize>>;

static DRAWS: &[usize] = &[49,48,98,84,71,59,37,36,6,21,46,30,5,33,3,62,63,45,43,35,65,77,57,75,19,44,4,76,88,92,12,27,7,51,14,72,96,9,0,17,83,64,38,95,54,20,1,74,69,80,81,56,10,68,42,15,99,53,93,94,47,13,29,34,60,41,82,90,25,85,78,91,32,70,58,28,61,24,55,87,39,11,79,50,22,8,89,26,16,2,73,23,18,66,52,31,86,97,67,40];

// TODO: Needs some optimization
fn part_1(input: &[Board]) -> usize {
    for i in 5..DRAWS.len() {
      let winner = input.iter()
        .find_map(|b| check_board(&DRAWS[0..i], &b));
      if let Some(score) = winner {
        return score;
      }
    }
    unreachable!()
}
  
// TODO: Needs a lot of optimization
fn part_2(input: &[Board]) -> usize {
    let mut input = input.iter().collect::<HashSet<_>>();
    for i in 5..DRAWS.len() {
        let winners = input.iter()
        .filter_map(|b| check_board(&DRAWS[0..i], &b).map(|score| (b.clone(),score)))
        .collect::<Vec<_>>();
        for (b,_) in &winners {
        input.remove(b);
        }
        if input.is_empty() {
        return winners[0].1
        }
    }
    unreachable!()
}
  

fn board_score(draws: &[usize], b: &Board) -> usize {
    b.iter().flatten().filter(|x| !draws.contains(x)).sum()
}
  
fn check_board(draws: &[usize], b: &Board) -> Option<usize> {
    for i in 0..5 {
      if (0..5).all(|j| draws.contains(&b[i][j])) {
        return Some(board_score(draws, b) * draws.last().unwrap());
      }
      if (0..5).all(|j| draws.contains(&b[j][i])) {
        return Some(board_score(draws, b) * draws.last().unwrap());
      }
    }
    None
}

pub(crate) fn run(buffer: String) -> crate::Result<RunData> {
    let start_setup = Instant::now();
    let input = buffer.split("\n\n")
    .map(|b| {
      b.lines()
        .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
        .collect()
    }).collect::<Vec<Board>>();
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
    output::print_day(4, "Giant Squid")?;
    output::print_part(1, "🦑 Final Score", &format!("{}", run_data.part_1))?;
    output::print_part(2, "🦑 Final Score", &format!("{}", run_data.part_2))?;
    output::print_timing(&run_data.times)?;
    Ok(())
}
