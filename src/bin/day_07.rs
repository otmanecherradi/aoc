use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "day_07";
const INPUT_FILE: &str = concatcp!("inputs/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();

        let mut pipes: HashSet<usize> = HashSet::new();
        let mut answer = 0;

        for line in &lines {
            let width = line.len();
            let mut new_pipes = pipes.clone();

            for (col, ch) in line.chars().enumerate() {
                match ch {
                    'S' => {
                        new_pipes.insert(col);
                    }
                    '^' if pipes.contains(&col) => {
                        answer += 1;
                        new_pipes.remove(&col);
                        if col > 0 {
                            new_pipes.insert(col - 1);
                        }
                        if col + 1 < width {
                            new_pipes.insert(col + 1);
                        }
                    }
                    _ => {}
                }
            }

            pipes = new_pipes;
        }

        Ok(answer)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();

        let mut counts: HashMap<usize, usize> = HashMap::new();

        for line in &lines {
            let width = line.len();
            let mut new_counts = counts.clone();

            for (col, ch) in line.chars().enumerate() {
                match ch {
                    'S' => {
                        *new_counts.entry(col).or_insert(0) += 1;
                    }
                    '^' => {
                        if let Some(&n) = counts.get(&col) {
                            let entry = new_counts.entry(col).or_insert(0);
                            *entry -= n;
                            if *entry == 0 {
                                new_counts.remove(&col);
                            }
                            if col > 0 {
                                *new_counts.entry(col - 1).or_insert(0) += n;
                            }
                            if col + 1 < width {
                                *new_counts.entry(col + 1).or_insert(0) += n;
                            }
                        }
                    }
                    _ => {}
                }
            }

            counts = new_counts;
        }

        let answer = counts.values().sum();

        Ok(answer)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
