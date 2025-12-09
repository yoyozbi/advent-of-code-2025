use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader.lines().flatten().collect_vec();
        let regex = Regex::new(r"(\d+|\+|\*)").unwrap();

        let mut splited = answer
            .iter()
            .map(|v| regex.find_iter(v).map(|v| v.as_str()).collect_vec())
            .collect_vec();

        let rows = splited.len();
        let cols = splited[0].len();

        // Transpose
        splited = (0..cols)
            .map(|col| (0..rows).map(|row| splited[row][col]).collect())
            .collect();

        let mut result = 0;

        for line in splited {
            let operation = line[line.len() - 1];
            let line_result = line
                .iter()
                .rev()
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(|f| f.parse::<usize>().unwrap())
                .reduce(|a, b| match operation {
                    "*" => a * b,
                    _ => a + b,
                })
                .unwrap();

            result += line_result;
        }

        Ok(result)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().flatten().collect_vec();

        // Normalize line lengths
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        for line in &mut lines {
            if line.len() < width {
                line.push_str(&" ".repeat(width - line.len()));
            }
        }

        let height = lines.len();
        let grid = lines
            .into_iter()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        // Find separators columns
        let is_sep = (0..width)
            .map(|i| (0..height).all(|j| grid[j][i].is_whitespace()))
            .collect_vec();

        let mut result = 0;
        let mut x = 0;
        while x < width {
            // Skip separator columns
            while is_sep[x] && x < width {
                x += 1;
            }

            if x >= width {
                break;
            }

            let col_start = x;
            while x < width && !is_sep[x] {
                x += 1;
            }

            let col_end = x;

            // Get operator
            let mut op = None;
            for cx in col_start..col_end {
                let c = grid[height-1][cx];
                if c == '*' || c == '+' {
                    op = Some(c);
                    break;
                }
            }

            let op = op.expect("No operator for this column");

            // Collect numbers
            let mut nums = Vec::new();
            for cx in col_start..col_end {
                let mut digits = String::new();

                for j in 0..(height - 1) {
                    let c = grid[j][cx];
                    if c.is_ascii_digit() {
                        digits.push(c);
                    }
                }

                if !digits.is_empty() {
                    nums.push(digits.parse::<usize>()?);
                }
            }

            let col_value = match op {
                '*' => nums.into_iter().product::<usize>(),
                '+' => nums.into_iter().sum(),
                _ => unreachable!(),
            };

            result += col_value;
        }

        Ok(result)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    println!("\n=== Real ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
