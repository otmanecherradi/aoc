use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("inputs/", "day-", DAY, ".txt");

const TEST_PART1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST_PART2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let lines = reader.lines().flatten();

        let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

        let mut answer = 0;

        for line in lines {
            for [f_number, l_number] in re.captures_iter(&line).map(|c| c.extract()).map(|c| (c.1))
            {
                answer += f_number.parse::<i32>()? * l_number.parse::<i32>()?;
            }
        }

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST_PART1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let lines = reader.lines().flatten();

        let re_instructions = Regex::new(r"mul\(\d+,\s*\d+\)|do\(\)|don't\(\)")?;
        let re_numbers = Regex::new(r"mul\((\d+),(\d+)\)")?;

        let mut answer = 0;

        let mut include = true;

        for line in lines {
            for captures in re_instructions.captures_iter(&line) {
                for cmds in captures.iter() {
                    for cmd in cmds.iter() {
                        if cmd.as_str() == "don't()" {
                            include = false;
                        }

                        if cmd.as_str() == "do()" {
                            include = true;
                        }

                        if include {
                            println!("{} included", cmd.as_str());

                            for [f_number, l_number] in re_numbers
                                .captures_iter(cmd.as_str())
                                .map(|c| c.extract())
                                .map(|c| (c.1))
                            {
                                answer += f_number.parse::<i32>()? * l_number.parse::<i32>()?;
                            }
                        }
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST_PART2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
