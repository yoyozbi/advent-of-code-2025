use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn get_neighbors_positions(
    i: usize,
    j: usize,
    line_number: usize,
    column_number: usize,
) -> Vec<(usize, usize)> {
    let mut values = HashSet::new();
    fn clamp(val: usize, max: usize) -> usize {
        val.clamp(0, max - 1)
    }

    values.insert((clamp(i + 1, line_number), j));
    values.insert((clamp(i.saturating_sub(1), line_number), j));
    values.insert((i, clamp(j + 1, column_number)));
    values.insert((i, clamp(j.saturating_sub(1), column_number)));
    values.insert((
        clamp(i.saturating_sub(1), line_number),
        clamp(j.saturating_sub(1), column_number),
    ));
    values.insert((
        clamp(i.saturating_sub(1), line_number),
        clamp(j + 1, column_number),
    ));
    values.insert((clamp(i + 1, line_number), clamp(j + 1, column_number)));
    values.insert((
        clamp(i + 1, line_number),
        clamp(j.saturating_sub(1), column_number),
    ));

    values.remove(&(i, j));

    values.into_iter().collect_vec()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<Vec<char>> = reader
            .lines()
            .map(Result::unwrap)
            .map(|s| s.chars().collect())
            .collect();

        let line_number = lines.len();
        let column_number = lines[0].len();

        let result = lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, &f)| f == '@')
                    .filter(|(j, _)| {
                        get_neighbors_positions(i, *j, line_number, column_number)
                            .into_iter()
                            .map(|(i, j)| lines[i][j])
                            .filter(|&char| char == '@')
                            .count()
                            < 4
                    })
                    .count()
            })
            .reduce(|a, b| a + b);

        match result {
            Some(value) => Ok(value),
            None => Err(anyhow!("failed to find a valid answer")),
        }
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines: Vec<Vec<char>> = reader
            .lines()
            .map(Result::unwrap)
            .map(|s| s.chars().collect())
            .collect();

        let line_number = lines.len();
        let column_number = lines[0].len();

        let mut result = 0;

        loop {
            let mut iteration_result = 0;
            let mut to_remove: Vec<(usize, usize)> = Vec::new();

            for (i, line) in lines.clone().into_iter().enumerate() {
                let to_change: Vec<_> = line
                    .iter()
                    .enumerate()
                    .filter(|(_, &f)| f == '@')
                    .filter(|(j, _)| {
                        get_neighbors_positions(i, *j, line_number, column_number)
                            .into_iter()
                            .map(|(i, j)| lines[i][j])
                            .filter(|&char| char == '@')
                            .count()
                            < 4
                    }).map(|(j, _)| (i,j)).collect_vec();

                iteration_result += to_change.len();

               to_remove.extend(to_change);
            }

            result += iteration_result;
            if iteration_result == 0 {
                break;
            }


            for (i,j) in to_remove {
                lines[i][j] = 'x';
            }


            //println!("------iteration({})--------------", iteration_result);
            //println!("{}", lines.iter().map(|line| line.iter().collect::<String>()).join("\n"));
        }

        Ok(result)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
