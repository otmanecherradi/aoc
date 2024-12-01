use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("inputs/", "day-", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let mut left_numbers = Vec::new();
        let mut right_numbers = Vec::new();

        let lines = reader.lines().flatten();

        let mut answer = 0;

        for line in lines {
            let mut numbers = line.split_whitespace();
            let left = numbers.next().unwrap().parse::<i64>().unwrap();
            let right = numbers.next().unwrap().parse::<i64>().unwrap();

            left_numbers.push(left);
            right_numbers.push(right);
        }

        left_numbers.sort();
        right_numbers.sort();

        for (left, right) in left_numbers.iter().zip(right_numbers.iter()) {
            let distance = cmp::max(left, right) - cmp::min(left, right);
            answer += distance;
        }

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let mut left_numbers = Vec::new();
        let mut right_counts = HashMap::new();

        let lines = reader.lines().flatten();

        let mut answer = 0;

        for line in lines {
            let mut numbers = line.split_whitespace();
            let left = numbers.next().unwrap().parse::<i64>().unwrap();
            let right = numbers.next().unwrap().parse::<i64>().unwrap();

            left_numbers.push(left);
            *right_counts.entry(right).or_insert(0) += 1;
        }

        for num in left_numbers {
            let right_count = right_counts.get(&num).unwrap_or(&0);
            answer += num * right_count;
        }

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
