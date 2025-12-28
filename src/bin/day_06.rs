use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "day_06";
const INPUT_FILE: &str = concatcp!("inputs/", DAY, ".txt");

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
        let lines: Vec<String> = reader.lines().flatten().collect();

        let ops_line = lines.last().unwrap();
        let data_lines = &lines[..lines.len() - 1];

        let operations: Vec<char> = ops_line.chars().filter(|c| !c.is_whitespace()).collect();
        let grid: Vec<Vec<usize>> = data_lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let answer = grid[0]
            .iter()
            .enumerate()
            .map(|(col_idx, _)| match operations[col_idx] {
                '+' => grid.iter().map(|row| row[col_idx]).sum::<usize>(),
                '*' => grid.iter().map(|row| row[col_idx]).product::<usize>(),
                _ => 0,
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();

        let ops_line = lines.last().unwrap();
        let data_lines = &lines[..lines.len() - 1];

        let operations: Vec<char> = ops_line
            .chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .collect();
        let data_rows: Vec<Vec<char>> = data_lines.iter().map(|r| r.chars().collect()).collect();

        let mut grid: Vec<Vec<Vec<char>>> = vec![];
        let mut current: Vec<Vec<char>> = vec![];

        for col_idx in (0..data_rows[0].len()).rev() {
            let pivot = data_rows.iter().map(|r| r[col_idx]).collect::<Vec<char>>();

            if pivot.iter().all(|c| c.is_whitespace()) {
                if !current.is_empty() {
                    grid.push(current.clone());
                    current.clear();
                }
                continue;
            }
            current.push(pivot);
        }
        if !current.is_empty() {
            grid.push(current);
        }

        let answer: usize = grid
            .iter()
            .zip(operations.iter())
            .map(|(grid_col, op)| {
                let numbers: Vec<usize> = grid_col
                    .iter()
                    .map(|col| {
                        col.iter()
                            .filter(|c| !c.is_whitespace())
                            .map(|c| c.to_digit(10).unwrap() as usize)
                            .fold(0, |acc, digit| acc * 10 + digit)
                    })
                    .collect();
                match op {
                    '+' => numbers.iter().sum::<usize>(),
                    '*' => numbers.iter().product::<usize>(),
                    _ => 0,
                }
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
