use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "day_03";
const INPUT_FILE: &str = concatcp!("inputs/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();

        let mut answer = 0;

        for value in lines {
            let digits = value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();

            let mut best = digits[0] - 0;
            let mut val = 0;

            for nbr in &digits[1..] {
                let diff = nbr - 0;
                let maybe_new_val = 10 * best + diff;
                if maybe_new_val > val {
                    val = maybe_new_val;
                }
                if diff > best {
                    best = diff;
                }
            }

            answer += val;
        }

        Ok(answer)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // //endregion

    // //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();

        const MAX_LEN: usize = 12;
        let mut answer = 0;

        for value in lines {
            let digits = value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();

            let n = digits.len();

            let mut pos = 0;
            let mut remaining_to_pick = cmp::min(MAX_LEN, n);

            let mut val = 0;
            while pos < n && remaining_to_pick > 0 {
                let search_end = n - remaining_to_pick;
                let mut best_idx = pos;

                for i in pos..=search_end {
                    if digits[i] > digits[best_idx] {
                        best_idx = i;
                    }
                }
                pos = best_idx + 1;
                remaining_to_pick -= 1;

                val = val * 10 + digits[best_idx];
            }

            answer += val;
        }

        Ok(answer)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // //endregion

    Ok(())
}
