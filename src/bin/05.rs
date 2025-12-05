use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten().collect_vec();
        let split_position = lines
            .iter()
            .position(|s| s.is_empty())
            .ok_or_else(|| anyhow!("no split in lines"))?
            + 1;
        let (fresh_ranges, to_check) = lines.split_at(split_position);

        let fresh_ranges = fresh_ranges
            .into_iter()
            .map(|s| s.to_string())
            .filter(|v| !v.is_empty())
            .map(|range| {
                range
                    .split('-')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec();

        let mut result = 0;
        for number in to_check.iter().map(|val| val.parse::<usize>().unwrap()) {
            for (start_range, end_range) in fresh_ranges.iter() {
                if number >= *start_range && number <= *end_range {
                    result += 1;
                    //println!("{}", number);
                    break;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten().collect_vec();

        let split_position = lines
            .iter()
            .position(|s| s.is_empty())
            .ok_or_else(|| anyhow!("no split in lines"))?
            + 1;
        let (fresh_ranges, _) = lines.split_at(split_position);
        let mut fresh_ranges = fresh_ranges
            .into_iter()
            .map(|s| s.to_string())
            .filter(|v| !v.is_empty())
            .map(|range| {
                range
                    .split('-')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_tuple::<(usize, usize)>()
                    .unwrap()
            })
            .collect_vec();

        fresh_ranges.sort_by(|a, b| a.cmp(b));

        let mut merged: Vec<(usize, usize)> = Vec::new();

        for range in fresh_ranges {
            if merged.is_empty() {
                merged.push(range);
            } else {
                let last = merged.last_mut().unwrap();

                // Check if overlaps
                if range.0 <= last.1+1 {
                    // Extend last element to this element length (We use max to account for fully overlapping ranges)
                    last.1 = last.1.max(range.1);
                } else {
                    merged.push(range);
                }
            }
        }

        /*println!(
            "{}",
            merged
                .iter()
                .map(|(start, end)| format!("({},{})", start, end))
                .join(",")
        );*/

        Ok(merged.iter().map(|r| (r.0..r.1).len() + 1).sum())
        //endregion
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
