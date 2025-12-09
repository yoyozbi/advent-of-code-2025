use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use memoize::memoize;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();
        let grid = lines
            .into_iter()
            .map(|f| f.chars().collect_vec())
            .collect_vec();
        let height = grid.len();
        let width = grid[0].len();

        let mut y = 1; // we skip the first line
        let mut beams_x_index = grid[0]
            .iter()
            .positions(|&s| s == 'S')
            .collect::<HashSet<usize>>();

        let mut total = 0;
        while y < height {
            let splitters = grid[y].iter().positions(|&s| s == '^').collect_vec();

            if splitters.is_empty() {
                y += 1;
                continue;
            }

            for split in splitters {
                if beams_x_index.contains(&split) {
                    beams_x_index.remove(&split);
                    let mut to_add = Vec::new();
                    if split > 0 {
                        to_add.push(split - 1);
                    }

                    if split < width - 1 {
                        to_add.push(split + 1);
                    }

                    total += 1;

                    beams_x_index.extend(to_add);
                }
            }

            y += 1;
        }

        Ok(total)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();
        let grid = lines
            .into_iter()
            .map(|f| f.chars().collect_vec())
            .collect_vec();

        let start_column = grid[0]
            .iter()
            .position(|&s| s == 'S')
            .expect("Failed to find start position");
        return Ok(ways(&grid, 0, start_column));

        #[memoize(Ignore: grid)]
        fn ways(grid: &Vec<Vec<char>>, row: usize, column: usize) -> usize {
            if row >= grid.len() {
                return 1;
            } else if column >= grid[0].len() {
                return 0;
            }

            match grid[row][column] {
                '.' | 'S' => ways(grid, row + 1, column),
                '^' => {
                    let mut result = 0;
                    if column + 1 < grid[0].len() {
                        result += ways(grid, row, column + 1);
                    }
                    if column > 0 {
                        result += ways(grid, row, column - 1);
                    }
                    result
                }
                _ => unreachable!(),
            }
        }
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
