use adv_code_2024::*;
use anyhow::*;
use std::collections::HashMap;

const DAY: &str = "01";

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

struct Day1;

impl Solution<i32> for Day1 {
    fn part1(&self, input: InputData) -> Option<i32> {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];

        for line in input.lines() {
            let nums = line
                .trim()
                .split(" ")
                .filter(|l| l.trim().len() > 0)
                .collect::<Vec<&str>>();
            left.push(nums.get(0).unwrap().parse::<i32>().unwrap());
            right.push(nums.get(1).unwrap().parse::<i32>().unwrap());
        }

        left.sort();
        right.sort();

        let answer = left
            .into_iter()
            .zip(right.into_iter())
            .map(|(l, r)| l - r)
            .map(|diff| diff.abs())
            .sum();

        Some(answer)
    }

    fn part2(&self, input: InputData) -> Option<i32> {
        let mut left: Vec<i32> = vec![];
        let mut right: HashMap<i32, i32> = HashMap::new();

        let it = input
            .lines()
            .map(|line| {
                line.split(" ")
                    .filter(|part| !part.is_empty())
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|f| (f.get(0).cloned().unwrap(), f.get(1).cloned().unwrap()));

        for (l, r) in it {
            left.push(l);
            right.entry(r).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut answer = 0;
        for l in left {
            answer += l * *right.get(&l).unwrap_or(&0);
        }

        Some(answer)
    }

    fn day() -> &'static str {
        DAY
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    let solution = Day1 {};

    solution.run_tests(TEST, 11, 31)?;
    solution.run_part_1()?;
    solution.run_part_2()?;

    Ok(())
}
