use adv_code_2024::*;
use anyhow::*;
use log::LevelFilter;
use std::{cmp::min, fmt::Debug, str::FromStr};

const DAY: &str = "02";

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

struct Day2;

fn split_line_in_vec<T>(line: &String) -> Vec<T>
where
    T: Default + Copy + FromStr + Debug,
{
    line.trim()
        .split(" ")
        .filter_map(|l| match l.parse::<T>() {
            std::result::Result::Ok(v) => Some(v),
            Err(_) => {
                panic!("Could not parse element: {}", l)
            }
        })
        .collect::<Vec<T>>()
}

fn check(columns: &Vec<i32>, j: usize, i: usize) -> (i32, i32) {
    let is_asc = columns
        .get(i)
        .and_then(|b| columns.get(j).map(|a| b > a))
        .expect("elements should exist");

    let is_small = columns
        .get(i)
        .and_then(|b| {
            columns
                .get(j)
                .map(|a| (b - a).abs() >= 1 && (b - a).abs() <= 3)
        })
        .expect("elements should exist");

    return (
        if is_asc && is_small { 0 } else { 1 },
        if !is_asc && is_small { 0 } else { 1 },
    );
}

impl Solution<i32> for Day2 {
    fn part1(&self, input: InputData) -> Option<i32> {
        let mut safe = 0;

        fn is_safe(columns: Vec<i32>) -> bool {
            let mut mem = [[0i32; 20]; 2];

            for i in 1..columns.len() {
                let (asc_inc, desc_inc) = check(&columns, i - 1, i);

                mem[0][i] = mem[0][i - 1] + asc_inc;
                mem[1][i] = mem[1][i - 1] + desc_inc;
            }

            let best_asc = mem[0][columns.len() - 1];
            let best_desc = mem[1][columns.len() - 1];

            return best_asc.min(best_desc) < 1;
        }
        for line in input.lines() {
            let columns = split_line_in_vec::<i32>(&line);
            if is_safe(columns) {
                safe += 1;
            }
        }

        Some(safe)
    }

    fn part2(&self, input: InputData) -> Option<i32> {
        let mut safe = 0;

        for line in input.lines() {
            let columns = split_line_in_vec::<i32>(&line);
            if is_safe_p2(&columns) {
                safe += 1;
            }
        }

        Some(safe)
    }

    fn day() -> &'static str {
        DAY
    }
}

/*
P2:

347 - too low
353 - wrong
*/

fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info)?;

    start_day(DAY);

    let solution = Day2 {};

    solution.run_tests_part1(TEST, 2);

    solution.run_part_1()?;

    solution.run_tests_part2(TEST, 4);

    solution.run_part_2()?;
    // solution.run_part_2()?;

    Ok(())
}

fn is_safe_p2(columns: &Vec<i32>) -> bool {
    println!(" === {:?} ===", columns);
    let mut mem = [[0i32; 20]; 2];

    // fn update(j:usize, i:usize, inc1:i32, )

    for i in 1..columns.len() {
        if i == 1 {
            let (asc_inc, desc_inc) = check(columns, i - 1, i);

            mem[0][i] = mem[0][i - 1] + asc_inc;
            mem[1][i] = mem[1][i - 1] + desc_inc;
        } else {
            let (asc_inc, desc_inc) = check(columns, i - 1, i);
            let (asc_inc2, desc_inc2) = check(columns, i - 2, i);

            // let min_inc1 = min(asc_inc, asc_inc2);
            // let no_skip1 = mem[0][i - 1] + asc_inc;
            // let skip1 = mem[0][i - 2] + asc_inc2;
            if asc_inc == 0 {
                mem[0][i] = mem[0][i - 1];
            } else if asc_inc2 == 0 {
                mem[0][i] = min(mem[0][i - 1], mem[0][i - 2]) + 1;
            } else {
                mem[0][i] = mem[0][i - 1] + 2;
            }

            if desc_inc == 0 {
                mem[1][i] = mem[1][i - 1];
            } else if desc_inc2 == 0 {
                mem[1][i] = min(mem[1][i - 1], mem[1][i - 2]) + 1;
            } else {
                mem[1][i] = mem[1][i - 1] + 2;
            }

            // let min_inc2 = min(desc_inc, desc_inc2);
        }
    }

    let last = columns.len() - 1;
    println!("asc: {:?}", &mem[0][0..=last]);
    println!("desc: {:?}", &mem[1][0..=last]);
    let best_asc = min(mem[0][last], mem[0][last - 1] + 1);
    let best_desc = min(mem[1][last], mem[1][last - 1] + 1);

    // let best_asc = if mem[0][last] == 2 && mem[0][last - 1] == 0 {
    //     0
    // } else {
    //     mem[0][last]
    // };
    // let best_desc = if mem[1][last] == 2 && mem[1][last - 1] == 0 {
    //     0
    // } else {
    //     mem[1][last]
    // };

    return best_asc.min(best_desc) <= 1;
}

#[cfg(test)]
mod test {

    use rand::Rng;

    use crate::is_safe_p2;

    #[test]
    fn test_tt() {
        let positive_cases = parse_test_case(
            "\
                1 3 6 7 9
                7 6 4 2 1
                1 3 2 4 5
                8 6 4 4 1
                3 6 9 12 15
                1 2 3 3 4
                1 2 3 4 100
                ",
        );

        for pc in positive_cases {
            assert!(is_safe_p2(&pc), "expected true got false for {:?}", pc);
        }

        let negative_cases = parse_test_case(
            "\
                1 2 7 8 9
                9 7 6 2 1
                2 2 2 3 4 5
                2 3 4 5 5 5
                1 10 2 3 4 100 5 6
                ",
        );

        // let negative_cases = parse_test_case(
        //     "\
        //         1 10 2 3 4 100 5 6
        //         ",
        // );

        for nc in negative_cases {
            assert!(!is_safe_p2(&nc), "expected false got true for {:?}", nc);
        }
    }

    #[test]
    fn test_tt_rand() {
        for _ in 0..100 {
            let columns = create_valid_list();
            assert!(
                is_safe_p2(&columns),
                "expected true got false for {:?}",
                columns
            );
        }
    }

    fn create_valid_list() -> Vec<i32> {
        let mut list = [1i32; 10];

        let bad_char: usize = rand::thread_rng().gen_range(0..10);

        for i in 1..list.len() {
            let valid_inc = rand::thread_rng().gen_range(1..4);
            if i == bad_char {
                list[i] = rand::thread_rng().gen_range(1..1000)
            } else if i >= 2 && i == bad_char + 1 {
                list[i] = list[i - 2] + valid_inc;
            } else {
                list[i] = list[i - 1] + valid_inc;
            }
        }

        list.to_vec()
    }

    fn parse_test_case(st: &'static str) -> impl Iterator<Item = Vec<i32>> {
        st.split("\n")
            .map(|l| l.trim())
            .filter(|line| line.len() > 0)
            .map(|line| {
                line.split(" ")
                    .map(|c| c.parse::<i32>().expect("expects number"))
                    .collect::<Vec<i32>>()
            })
    }
}
