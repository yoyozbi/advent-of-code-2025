use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Position;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::absolute;
use std::thread::current;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        let regex_line = Regex::new("([RL])([0-9]+)")?;

        let mut current_dial_number: i32 = 50;
        let mut number_of_zero = 0;

        println!("====================================");

        for line in lines {
            let line = line?;

            let matches = regex_line.captures(&line).unwrap();
            let direction: i32 = match matches.get(1).unwrap().as_str() {
                "R" => 1,
                "L" => -1,
                _ => unreachable!(),
            };

            let number = matches.get(2).unwrap().as_str().parse::<i32>()?;

            let mut new_dial_number = (current_dial_number + (direction * number)) % 100;
            if new_dial_number < 0 {
                new_dial_number += 100;
            }

            current_dial_number = new_dial_number;

            if current_dial_number == 0 {
                number_of_zero += 1;
            }

        }

        Ok(number_of_zero)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        let regex_line = Regex::new("([RL])([0-9]+)")?;

        let mut current_dial_number: i32 = 50;
        let mut number_of_zero = 0;

        println!("====================================");

        for line in lines {
            let line = line?;

            let matches = regex_line.captures(&line).unwrap();
            let direction: i32 = match matches.get(1).unwrap().as_str() {
                "R" => 1,
                "L" => -1,
                _ => unreachable!(),
            };

            let number = matches.get(2).unwrap().as_str().parse::<i32>()?;


            let mut new_dial_number = current_dial_number;
            for _ in 0..number {
                new_dial_number += direction;
               if(new_dial_number == 0)  {
                   number_of_zero += 1;
               }else if(new_dial_number < 0) {
                   new_dial_number = 99;
               }else if(new_dial_number > 99) {
                   number_of_zero += 1;
                   new_dial_number = 0;
               }
            }


            current_dial_number = new_dial_number;

            println!("{} dial: {}", line, new_dial_number);
        }

        Ok(number_of_zero)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
