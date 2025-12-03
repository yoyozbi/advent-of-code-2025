use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }

            let numbers = line
                .split("")
                .filter(|s| !s.is_empty())
                .map(&str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            let first_max = numbers[..numbers.len() - 1].iter().max().unwrap();
            let first_position = numbers.iter().position(|f| f == first_max).unwrap();
            let second_max = numbers[first_position + 1..].iter().max().unwrap();

            let concatenated = format!("{}{}", first_max, second_max).parse::<usize>()?;

            result += concatenated;
        }

        Ok(result)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        const NUM_TO_HAVE: usize = 12;
        let mut result = 0;

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }

            let mut numbers = line
                .split("")
                .filter(|s| !s.is_empty())
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            let mut digits = Vec::new();
            let mut start_position = 0;

            for i in 0..12 {
                let end_index = numbers.len() - NUM_TO_HAVE + i;

                let max_value = numbers[start_position..=end_index]
                    .iter()
                    .max()
                    .copied()
                    .unwrap();

                digits.push(max_value);

                start_position = numbers[start_position..=end_index]
                    .iter()
                    .position(|&f| f == max_value)
                    .unwrap()
                    + start_position
                    + 1;
            }

            assert_eq!(NUM_TO_HAVE, digits.len());
            let concatenated = digits.into_iter().join("").parse::<usize>()?;
            result += concatenated;
            println!("{} -> {} = {}", line, concatenated, result);
        }

        Ok(result)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
