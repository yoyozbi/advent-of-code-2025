use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use good_lp::*;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let machines = reader.lines().flatten().collect_vec();
        let main_re = Regex::new(r"\[([.#]+)\]\s+((?:\([0-9,]+\)\s*)+)\{([0-9,]+)\}")?;
        let buttons_re = Regex::new(r"\(([0-9,]+)\)")?;

        let mut operations_per_machine = Vec::new();
        for machine in machines.iter() {
            if let Some(captures) = main_re.captures(&machine) {
                let expected_signal = captures[1]
                    .to_string()
                    .trim()
                    .split("")
                    .map(|f| {
                        if f == "." {
                            "0"
                        } else if f == "#" {
                            "1"
                        } else {
                            ""
                        }
                    })
                    .join("");

                let signal_len = captures[1].len();

                let expected_signal = u32::from_str_radix(&expected_signal, 2)?;

                let buttons_section = &captures[2];
                let buttons = buttons_re
                    .captures_iter(&buttons_section)
                    .map(|f| {
                        f.iter()
                            .skip(1)
                            .flat_map(|b| {
                                b.unwrap()
                                    .as_str()
                                    .split(",")
                                    .map(|b| b.parse::<usize>().unwrap())
                            })
                            .collect_vec()
                    })
                    .collect_vec();

                let buttons_masks = buttons
                    .iter()
                    .map(|f| {
                        f.into_iter()
                            .map(|b| (1_u32 << (signal_len - 1)) >> b)
                            .reduce(|a, b| a | b)
                            .unwrap()
                    })
                    .collect_vec();

                // println!(
                //     "Expected signals: {}",
                //     format_int(&expected_signal, &signal_len),
                // );
                // println!("Buttons: {:?}", buttons);
                // println!(
                //     "Mask: {}",
                //     buttons_masks
                //         .iter()
                //         .map(|f| format_int(&f, &signal_len))
                //         .join(",")
                // );

                'outer: for operation_number in 1..12 {
                    for combos in buttons_masks.iter().combinations(operation_number) {
                        let mut current_signals = 0_u32;
                        for num in combos.iter() {
                            current_signals ^= *num;
                        }
                        // println!(
                        //     "[{}] => {}",
                        //     combos.iter().map(|f| format_int(&f, &signal_len)).join(","),
                        //     format_int(&current_signals, &signal_len)
                        // );

                        if current_signals == expected_signal {
                            operations_per_machine.push(operation_number);
                            break 'outer;
                        }
                    }
                }

                //println!("{:?}", operations_per_machine);
            }
        }

        assert_eq!(machines.len(), operations_per_machine.len());
        Ok(operations_per_machine.iter().sum())
    }

    fn format_int(value: &u32, size: &usize) -> String {
        format!("{:0width$b}", value, width = size)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let machines = reader.lines().flatten().collect_vec();
        let main_re = Regex::new(r"\[([.#]+)\]\s+((?:\([0-9,]+\)\s*)+)\{([0-9,]+)\}")?;
        let buttons_re = Regex::new(r"\(([0-9,]+)\)")?;

        let mut operations_per_machine = Vec::new();
        for machine in machines.iter() {
            if let Some(captures) = main_re.captures(&machine) {
                let expected_signal = captures[3]
                    .to_string()
                    .trim()
                    .split(",")
                    .map(|b| b.parse::<u32>().unwrap())
                    .collect_vec();

                let buttons_section = &captures[2];
                let buttons = buttons_re
                    .captures_iter(&buttons_section)
                    .map(|f| {
                        f.iter()
                            .skip(1)
                            .flat_map(|b| {
                                b.unwrap()
                                    .as_str()
                                    .split(",")
                                    .map(|b| b.parse::<usize>().unwrap())
                            })
                            .collect_vec()
                    })
                    .collect_vec();

                println!("Expected signals: {:?}", &expected_signal);
                println!("Buttons: {:?}", buttons);

                let mut problem = ProblemVariables::new();

                let button_vars: Vec<Variable> = (0..buttons.len())
                    .map(|i| problem.add(variable().min(0).integer()))
                    .collect();

                let objective = button_vars
                    .iter()
                    .fold(Expression::from(0), |acc, &var| acc + var);

                let mut solver = problem.minimise(objective).using(default_solver);
                for (counter_idx, &target) in expected_signal.iter().enumerate() {
                    let mut expr = Expression::from(0);
                    for (button_idx, button) in buttons.iter().enumerate() {
                        if button.contains(&counter_idx) {
                            expr = expr + button_vars[button_idx];
                        }
                    }
                    solver = solver.with(expr.eq(target as i32));
                }

                let solution = solver.solve()?;

                let total_presses: usize = button_vars
                    .iter()
                    .map(|&var| solution.value(var) as usize)
                    .sum();

                println!("Minimum presses: {}", total_presses);
                operations_per_machine.push(total_presses);
            }
        }

        assert_eq!(machines.len(), operations_per_machine.len());
        Ok(operations_per_machine.iter().sum::<usize>())
    }

    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
