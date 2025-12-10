use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Vector2 {
    x: usize,
    y: usize,
}

impl Vector2 {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn area_with(&self, other: &Self) -> usize {
        let width = self.x.max(other.x) - self.x.min(other.x) + 1;
        let height = self.y.max(other.y) - self.y.min(other.y) + 1;
        width * height
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    // TEST
    let vec1 = Vector2::new(2, 5);
    let vec2 = Vector2::new(11, 1);

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let vectors = reader
            .lines()
            .flatten()
            .map(|f| {
                let (x, y) = f.split_once(',').unwrap();

                Vector2::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect_vec();

        let (i, j, surface) = vectors
            .iter()
            .enumerate()
            .map(|(i, vec)| {
                vectors
                    .iter()
                    .enumerate()
                    .map(|(j, other)| (i, j, vec.area_with(other)))
                    .max_by_key(|(_, _, area)| *area)
                    .unwrap()
            })
            .max_by_key(|(_, _, area)| *area)
            .unwrap();

        println!("Max at ({}, {}) = {}", vectors[i], vectors[j], surface);

        Ok(surface)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        const RED: char = '#';
        const RED_STRING: &str = "#";
        const GREEN: char = 'X';
        const GREEN_STRING: &str = "X";
        const NOTHING: char = '.';
        const NOTHING_STRING: &str = ".";

        let mut vectors = reader
            .lines()
            .flatten()
            .map(|f| {
                let (x, y) = f.split_once(',').unwrap();

                Vector2::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect_vec();

        let max_vector_x = vectors.iter().max_by_key(|&x| x.x).unwrap().x;
        let min_vector_x = vectors.iter().min_by_key(|&x| x.x).unwrap().x;
        let max_vector_y = vectors.iter().max_by_key(|&x| x.y).unwrap().y;
        let min_vector_y = vectors.iter().min_by_key(|&x| x.y).unwrap().y;

        let delta_x = max_vector_x - min_vector_x + 1;
        let delta_y = max_vector_y - min_vector_y + 1;

        vectors = vectors
            .iter()
            .map(|vec| Vector2::new(vec.x - min_vector_x, vec.y - min_vector_y))
            .collect_vec();

        let mut grid = vec![String::from(NOTHING_STRING.repeat(delta_x + 1)); delta_y];

        //let mut grid = vec![NOTHING.repeat(max_vector_x + 1); max_vector_y + 1];

        for vector in vectors.iter() {
            grid[vector.y].replace_range(vector.x..=vector.x, GREEN_STRING)
        }

        // Bounding box
        for i in 0..vectors.len() {
            let vec1 = vectors[i];
            let vec2 = if i == vectors.len() - 1 {
                vectors[0]
            } else {
                vectors[i + 1]
            };

            // Vertical line
            if vec1.x == vec2.x {
                let min = vec1.y.min(vec2.y);
                let max = vec1.y.max(vec2.y);
                for y in (min + 1)..max {
                    grid[y].replace_range(vec1.x..=vec1.x, GREEN_STRING);
                }
            } else if vec1.y == vec2.y {
                let min = vec1.x.min(vec2.x);
                let max = vec1.x.max(vec2.x);
                grid[vec1.y]
                    .replace_range((min + 1)..max, GREEN_STRING.repeat(max - min - 1).as_str());
            }
        }

        println!(
            "Computing intervals for {} lines and {} cols",
            grid.len(),
            grid[0].len()
        );
        for y in 0..grid.len() {
            let line = &grid[y];

            if let (Some(first), Some(last)) =
                (line.find(|c| c != NOTHING), line.rfind(|c| c != NOTHING))
            {
                grid[y].replace_range(first..=last, GREEN_STRING.repeat(last - first).as_str());
            }
        }

        //println!("{}", grid.iter().join("\n"));

        // This takes 9+ minutes on the input...
        println!("Finding best surface");
        let best_surface = vectors
            .par_iter()
            .map(|first_vec| {
                let mut best_area = 0;
                for second_vec in vectors.iter().filter(|f| *f != first_vec) {
                    if first_vec.x == second_vec.x || first_vec.y == second_vec.y {
                        continue;
                    }

                    let top = first_vec.y.min(second_vec.y);
                    let bot = second_vec.y.max(first_vec.y);
                    let left = second_vec.x.min(first_vec.x);
                    let right = first_vec.x.max(second_vec.x);

                    let are_all_inside = (top..=bot)
                        .map(|yy| {
                            let line = &grid[yy][left..=right];
                            line == GREEN_STRING.repeat(line.len())
                        })
                        .all(|f| f);

                    if are_all_inside {
                        best_area = first_vec.area_with(second_vec).max(best_area);
                    }
                }
                best_area
            })
            .max();

        match best_surface {
            Some(surface) => Ok(surface),
            None => Err(anyhow!("No surface")),
        }
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
