use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Get all data
        let data = reader.lines().flatten().join("");

        let split_data = data.split(',');

        let mut result = 0;

        for range in split_data {
            let (start, end) = range
                .split('-')
                .map(|f| f.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            for i in start..=end {
                if i < 10 {
                    continue;
                }

                let str_i = i.to_string();
                if str_i.len() % 2 != 0 {
                    continue;
                }

                let (begin_number, end_number) = str_i.split_at(str_i.len() / 2);
                if begin_number == end_number {
                    result += i;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let data = reader.lines().flatten().join("");

        let split_data = data.split(',');

        let mut result = 0;

        for range in split_data {
            let (start, end) = range
                .split('-')
                .map(|f| f.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            for i in start..=end {
                if i < 10 {
                    continue;
                }

                let str_i = i.to_string();
                let max_possible_splits = str_i.len()/2;

                for j in 1..=max_possible_splits {
                    if str_i
                        .as_bytes()
                        .chunks(j)
                        .map(str::from_utf8)
                        .map(|f| f.unwrap().parse::<i32>().unwrap())
                        .all_equal()
                    {
                        result += i;
                        break;
                    }
                }
            }
        }

        Ok(result)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
