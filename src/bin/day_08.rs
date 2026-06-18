use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "day_08";
const INPUT_FILE: &str = concatcp!("inputs/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let points: Vec<[i64; 3]> = reader
            .lines()
            .flatten()
            .map(|line| {
                let mut it = line.split(',').filter_map(|s| s.parse().ok());
                [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
            })
            .collect();

        let connections = if points.len() <= 20 { 10 } else { 1000 };

        let mut pairs: Vec<(i64, usize, usize)> = Vec::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let d: i64 = (0..3).map(|k| (points[i][k] - points[j][k]).pow(2)).sum();
                pairs.push((d, i, j));
            }
        }
        pairs.sort_unstable();

        let mut parent: Vec<usize> = (0..points.len()).collect();
        let mut size = vec![1usize; points.len()];

        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }

        for &(_, i, j) in pairs.iter().take(connections) {
            let (a, b) = (find(&mut parent, i), find(&mut parent, j));
            if a != b {
                parent[a] = b;
                size[b] += size[a];
            }
        }

        let mut sizes: Vec<usize> = (0..points.len())
            .filter(|&i| find(&mut parent, i) == i)
            .map(|i| size[i])
            .collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));

        Ok(sizes.iter().take(3).product())
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let points: Vec<[i64; 3]> = reader
            .lines()
            .flatten()
            .map(|line| {
                let mut it = line.split(',').filter_map(|s| s.parse().ok());
                [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
            })
            .collect();

        let mut pairs: Vec<(i64, usize, usize)> = Vec::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let d: i64 = (0..3).map(|k| (points[i][k] - points[j][k]).pow(2)).sum();
                pairs.push((d, i, j));
            }
        }
        pairs.sort_unstable();

        let mut parent: Vec<usize> = (0..points.len()).collect();

        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }

        let mut components = points.len();
        for &(_, i, j) in &pairs {
            let (a, b) = (find(&mut parent, i), find(&mut parent, j));
            if a != b {
                parent[a] = b;
                components -= 1;
                if components == 1 {
                    return Ok((points[i][0] * points[j][0]) as usize);
                }
            }
        }

        bail!("never formed a single circuit");
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
