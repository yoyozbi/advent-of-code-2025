use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum GridValues {
    NOTHING,
    RED,
    GREEN,
}
impl Display for GridValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridValues::NOTHING => ".",
                GridValues::RED => "#",
                GridValues::GREEN => "X",
            }
        )
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
        let vectors = reader
            .lines()
            .flatten()
            .map(|f| {
                let (x, y) = f.split_once(',').unwrap();

                Vector2::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect_vec();

        let max_vector_x = vectors.iter().max_by_key(|&x| x.x).unwrap().x;
        let max_vector_y = vectors.iter().max_by_key(|&x| x.y).unwrap().y;

        let mut grid = vec![vec![GridValues::NOTHING; max_vector_x + 1]; max_vector_y + 1];

        for vector in vectors.iter() {
            grid[vector.y][vector.x] = GridValues::RED;
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
                    grid[y][vec1.x] = GridValues::GREEN;
                }
            } else if vec1.y == vec2.y {
                let min = vec1.x.min(vec2.x);
                let max = vec1.x.max(vec2.x);
                for x in (min + 1)..max {
                    grid[vec1.y][x] = GridValues::GREEN;
                }
            }
        }

        for y in 0..grid.len() {
            let mut is_inside = false;
            for x in 0..grid[y].len() {
                let cell = grid[y][x];
                match cell {
                    GridValues::NOTHING => {
                        if is_inside {
                            grid[y][x] = GridValues::GREEN;
                        }
                    }
                    GridValues::GREEN | GridValues::RED => {
                        is_inside = ((x + 1)..grid[y].len())
                            .map(|xx| grid[y][xx])
                            .any(|f| f == GridValues::RED || f == GridValues::GREEN)
                    }
                }
            }
        }

        println!("{}", grid.iter().map(|f| f.iter().join("")).join("\n"));

        let is_inside = grid
            .iter()
            .map(|r| {
                r.iter()
                    .map(|val| *val != GridValues::NOTHING)
                    .collect_vec()
            })
            .collect_vec();

        // Build prefix
        let mut pref = vec![vec![0; grid[0].len()]; grid.len()];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                pref[y][x] = if is_inside[y][x] { 1 } else { 0 };
                if y > 0 {
                    pref[y][x] += pref[y - 1][x];
                }
                if x > 0 {
                    pref[y][x] += pref[y][x - 1];
                }
                if y > 0 && x > 0 {
                    pref[y][x] -= pref[y - 1][x - 1];
                }
            }
        }

        let mut best_surface = 0;

        for first_vec in vectors.iter() {
            for second_vec in vectors.iter().filter(|f| *f != first_vec) {
                let rect_sum = |a: Vector2, b: Vector2| {
                    let lx = a.x.min(b.x);
                    let rx = a.x.max(b.x);
                    let ty = a.y.min(b.y);
                    let by = a.y.max(b.y);

                    let mut res = pref[by][rx] as isize;
                    if ty > 0 {
                        res -= pref[ty - 1][rx] as isize;
                    }
                    if lx > 0 {
                        res -= pref[by][lx - 1] as isize;
                    }
                    if ty > 0 && lx > 0 {
                        res += pref[ty - 1][lx - 1] as isize;
                    }
                    res
                };

                let area = first_vec.area_with(second_vec);
                if rect_sum(first_vec.clone(), second_vec.clone()) == area as isize {
                    best_surface = area.max(best_surface);
                }
            }
        }

        Ok(best_surface)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
