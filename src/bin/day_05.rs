use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("inputs/", "day-", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let lines = reader.lines().flatten();

        let mut answer = 0;

        let mut matrix: Vec<Vec<char>> = vec![];

        for line in lines {
            let row: Vec<char> = line.chars().collect();
            matrix.push(row);
        }

        let rows_count = matrix.len() as i32;
        let cols_count = matrix[0].len() as i32;

        for row_idx in 0..rows_count {
            for col_idx in 0..cols_count {
                if matrix[row_idx as usize][col_idx as usize] == 'X' {
                    for (row_direction, col_direction) in [
                        (0, 1),
                        (0, -1),
                        (1, 0),
                        (-1, 0),
                        (1, 1),
                        (-1, -1),
                        (1, -1),
                        (-1, 1),
                    ] {
                        let mut r = row_idx;
                        let mut c = col_idx;
                        let mut match_found = true;

                        for ch in "MAS".chars() {
                            r += row_direction;
                            c += col_direction;
                            if r < 0
                                || r >= rows_count
                                || c < 0
                                || c >= cols_count
                                || matrix[r as usize][c as usize] != ch
                            {
                                match_found = false;
                                break;
                            }
                        }

                        if match_found {
                            answer += 1;
                        }
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");

    // fn part2<R: BufRead>(reader: R) -> Result<i32> {
    //     let lines = reader.lines().flatten();

    //     let mut answer = 0;

    //     let mut matrix: Vec<Vec<char>> = vec![];

    //     for line in lines {
    //         let row: Vec<char> = line.chars().collect();
    //         matrix.push(row);
    //     }

    //     let rows_count = matrix.len() as i32;
    //     let cols_count = matrix[0].len() as i32;

    //     for row_idx in 1..rows_count - 1 {
    //         for col_idx in 1..cols_count - 1 {
    //             if matrix[row_idx as usize][col_idx as usize] == 'A' {
    //                 let element = [(-1, -1), (-1, 1), (1, 1), (1, -1)]
    //                     .map(|(dr, dc)| matrix[(row_idx + dr) as usize][(col_idx + dc) as usize])
    //                     .iter()
    //                     .collect();

    //                 if [
    //                     String::from("MSSM"),
    //                     String::from("SSMM"),
    //                     String::from("SMMS"),
    //                     String::from("MMSS"),
    //                 ]
    //                 .contains(&element)
    //                 {
    //                     answer += 1;
    //                 }
    //             }
    //         }
    //     }

    //     Ok(answer)
    // }

    // assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
