use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[derive(Debug, Clone)]
struct Region {
    width: u32,
    height: u32,
    shapes_to_fit: Vec<u32>,
}

impl Region {
    fn new(width: u32, height: u32, shapes_to_fit: Vec<u32>) -> Self {
        Self {
            width,
            height,
            shapes_to_fit,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten().collect_vec();
        let size_regex = Regex::new(r"(\d+)x(\d+)")?;

        let mut shapes = Vec::new();
        let mut regions = Vec::new();
        for (key, chunk) in &lines.into_iter().chunk_by(|line| line.is_empty()) {
            if key {
                continue;
            }

            let group = chunk.collect_vec();

            let first_chunk = group[0].as_str();

            if size_regex.is_match(first_chunk) {
                for line in group {
                    let (shape, other) = line.split_once(':').unwrap();
                    let (width, height) = shape.split_once('x').unwrap();
                    let (width, height) = (width.parse::<u32>()?, height.parse::<u32>()?);
                    let shapes_to_fit = other
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect_vec();
                    regions.push(Region::new(width, height, shapes_to_fit));
                }
            } else {
                let shape = group[1..].join("\n");
                shapes.push(shape);
            }
        }

        let shape_sizes = shapes.iter().map(|s| s.chars().filter(|c| *c == '#').count()).collect_vec();

        let mut result = 0;
        for region in regions {
            let total_shapes_area: usize = region.shapes_to_fit.iter().enumerate().map(|(i, s)| shape_sizes[i] * (*s) as usize).sum();
            if total_shapes_area + 3 <= region.area() as usize {
                result += 1;
            }
        }


        Ok(result)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
