use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "day_01";
const INPUT_FILE: &str = concatcp!("inputs/", DAY, ".txt");

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
        let lines = reader.lines().flatten();

        let mut answer = 0;
        let mut current_position: i16 = 50;

        for line in lines {
            let first_char = line.chars().next().unwrap();
            let number: i16 = line[1..].parse().unwrap();

            let delta = if first_char == 'L' { -number } else { number };
            current_position = (current_position + delta).rem_euclid(100);

            if current_position == 0 {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();

        let mut answer = 0;
        let mut current_position: i16 = 50;

        for line in lines {
            let first_char = line.chars().next().unwrap();
            let number: i16 = line[1..].parse().unwrap();

            let delta = if first_char == 'L' { -number } else { number };
            let target = if first_char == 'L' {
                current_position.rem_euclid(100)
            } else {
                (-current_position).rem_euclid(100)
            };

            let zero_count = if target == 0 { 100 } else { target };
            if number >= zero_count {
                answer += 1 + ((number - zero_count) / 100) as usize;
            }

            current_position = (current_position + delta).rem_euclid(100);
        }

        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
