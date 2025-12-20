use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "day_05";
const INPUT_FILE: &str = concatcp!("inputs/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();

        let mut ranges: Vec<(usize, usize)> = vec![];
        let mut numbers: Vec<usize> = vec![];

        for line in lines {
            if line.is_empty() {
                continue;
            }

            if line.contains("-") {
                let parts: Vec<&str> = line.split('-').collect();
                let first_nbr = parts[0].parse::<usize>()?;
                let second_nbr = parts[1].parse::<usize>()?;

                ranges.push((first_nbr, second_nbr));
            } else {
                let nbr = line.parse::<usize>()?;
                numbers.push(nbr);
            }
        }

        let mut answer = 0;

        for nbr in numbers {
            for (start, end) in &ranges {
                if nbr >= *start && nbr <= *end {
                    answer += 1;
                    break;
                }
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
        let lines: Vec<String> = reader.lines().flatten().collect();

        let mut ranges: Vec<(usize, usize)> = vec![];

        for line in lines {
            if line.contains("-") {
                let parts: Vec<&str> = line.split('-').collect();
                let first_nbr = parts[0].parse::<usize>()?;
                let second_nbr = parts[1].parse::<usize>()?;

                ranges.push((first_nbr, second_nbr));
            }
        }

        ranges.sort();
        let mut merged: Vec<(usize, usize)> = vec![];

        for (start, end) in ranges {
            if let Some(last) = merged.last_mut() {
                if start <= last.1 + 1 {
                    last.1 = last.1.max(end);
                    continue;
                }
            }
            merged.push((start, end));
        }

        let answer = merged.iter().map(|(start, end)| end - start + 1).sum();

        Ok(answer)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
