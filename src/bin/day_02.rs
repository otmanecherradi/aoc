use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "day_02";
const INPUT_FILE: &str = concatcp!("inputs/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();
        let first_line = lines.into_iter().next().unwrap();

        let values = first_line
            .split(',')
            .map(|s| {
                let mut parts = s.split('-');
                let start = parts.next().unwrap();
                let end = parts.next().unwrap();
                (start, end)
            })
            .collect::<Vec<(&str, &str)>>();

        let mut answer = 0;
        let mut seen_values: HashSet<usize> = HashSet::new();

        for (start_str, end_str) in values {
            let max_digits = end_str.len();

            let start_nbr = start_str.parse().unwrap();
            let end_nbr = end_str.parse().unwrap();

            for k in 1..=(max_digits / 2) {
                let pow10_k = 10usize.pow(k as u32);
                let multiplier = pow10_k + 1;

                let lower_bound = if k == 1 {
                    1
                } else {
                    10usize.pow((k - 1) as u32)
                };
                let upper_bound = pow10_k - 1;

                let min_h = (start_nbr + multiplier - 1) / multiplier;
                let max_h = end_nbr / multiplier;

                let low = lower_bound.max(min_h);
                let high = upper_bound.min(max_h);

                if low > high {
                    continue;
                }

                for digit in low..=high {
                    let v = digit * multiplier;
                    if v >= start_nbr && v <= end_nbr {
                        if seen_values.insert(v) {
                            answer += v;
                        }
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // //endregion

    // //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();
        let first_line = lines.into_iter().next().unwrap();

        let values = first_line
            .split(',')
            .map(|s| {
                let mut parts = s.split('-');
                let start = parts.next().unwrap();
                let end = parts.next().unwrap();
                (start, end)
            })
            .collect::<Vec<(&str, &str)>>();

        let mut answer = 0;
        let mut seen_values: HashSet<usize> = HashSet::new();

        for (start_str, end_str) in values {
            let max_digits = end_str.len();
            let start_nbr = start_str.parse().unwrap();
            let end_nbr = end_str.parse().unwrap();

            for k in 1..=max_digits {
                for repeat_count in 2..=(max_digits / k) {
                    let total_digits = k * repeat_count;

                    if total_digits > max_digits {
                        break;
                    }

                    let mut multiplier = 0usize;
                    for i in 0..repeat_count {
                        multiplier += 10usize.pow((i * k) as u32);
                    }

                    let lower_bound = if k == 1 {
                        1
                    } else {
                        10usize.pow((k - 1) as u32)
                    };
                    let upper_bound = 10usize.pow(k as u32) - 1;

                    let min_h = (start_nbr + multiplier - 1) / multiplier;
                    let max_h = end_nbr / multiplier;

                    let low = lower_bound.max(min_h);
                    let high = upper_bound.min(max_h);

                    if low > high {
                        continue;
                    }

                    for digit in low..=high {
                        let value = digit * multiplier;
                        if value >= start_nbr && value <= end_nbr {
                            if seen_values.insert(value) {
                                answer += value;
                            }
                        }
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // //endregion

    Ok(())
}
