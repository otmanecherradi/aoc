use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("inputs/", "day-", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn is_safe(numbers: &[i64]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff < -3 || diff > 3 {
            return false;
        }
        is_increasing &= diff >= 1;
        is_decreasing &= diff <= -1;
    }

    is_increasing || is_decreasing
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let lines = reader.lines().flatten();

        let mut answer = 0;

        for line in lines {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            if is_safe(&numbers) {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let lines = reader.lines().flatten();

        let mut answer = 0;

        for line in lines {
            let report: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            if is_safe(&report) {
                answer += 1;
            } else {
                let mut maybe_safe = false;

                for idx in 0..report.len() {
                    let mut new_report = report.clone();
                    new_report.remove(idx);

                    if is_safe(&new_report) {
                        maybe_safe = true;
                        break;
                    }
                }
                if maybe_safe {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
