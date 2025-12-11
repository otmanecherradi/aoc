use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

const DAY: &str = "day_04";
const INPUT_FILE: &str = concatcp!("inputs/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();
        let rows = lines.len();

        let mut answer = 0;

        for (row_idx, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();

            for (col_idx, &ch) in chars.iter().enumerate() {
                if ch == '@' {
                    let mut adjacent_count = 0;

                    for (dr, dc) in DIRECTIONS {
                        let nr = row_idx as i32 + dr;
                        let nc = col_idx as i32 + dc;

                        if nr >= 0 && nr < rows as i32 && nc >= 0 {
                            let nr = nr as usize;
                            let nc = nc as usize;

                            if nr < lines.len() && nc < lines[nr].len() {
                                if lines[nr].chars().nth(nc) == Some('@') {
                                    adjacent_count += 1;
                                }
                            }
                        }
                    }

                    if adjacent_count < 4 {
                        answer += 1;
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // //endregion

    // //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines: Vec<String> = reader.lines().flatten().collect();
        let rows = lines.len();

        let mut answer = 0;

        loop {
            let mut to_remove = Vec::new();

            for (row_idx, line) in lines.iter().enumerate() {
                let chars: Vec<char> = line.chars().collect();

                for (col_idx, &ch) in chars.iter().enumerate() {
                    if ch == '@' {
                        let mut adjacent_count = 0;

                        for (dr, dc) in DIRECTIONS {
                            let nr = row_idx as i32 + dr;
                            let nc = col_idx as i32 + dc;

                            if nr >= 0 && nr < rows as i32 && nc >= 0 {
                                let nr = nr as usize;
                                let nc = nc as usize;

                                if nr < lines.len() && nc < lines[nr].len() {
                                    if lines[nr].chars().nth(nc) == Some('@') {
                                        adjacent_count += 1;
                                    }
                                }
                            }
                        }

                        if adjacent_count < 4 {
                            to_remove.push((row_idx, col_idx));
                        }
                    }
                }
            }

            if to_remove.is_empty() {
                break;
            }

            for (row_idx, col_idx) in to_remove {
                let mut chars: Vec<char> = lines[row_idx].chars().collect();
                chars[col_idx] = '.';
                lines[row_idx] = chars.into_iter().collect();
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // //endregion

    Ok(())
}
