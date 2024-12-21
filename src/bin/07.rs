use std::collections::VecDeque;

use adv_code_2024::*;
use anyhow::*;
use itertools::Itertools;
use log::LevelFilter;
const DAY: &str = "07";

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

struct Day7;

fn parse_data(input: InputData) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|l| l.trim().split_once(":"))
        .filter_map(|maybe_sp| {
            maybe_sp.map(|sp| {
                (
                    sp.0.trim().parse::<i64>().expect(
                        format!("left side value is an int, found {}", sp.0.trim()).as_str(),
                    ),
                    sp.1.trim()
                        .split(' ')
                        .map(|n| n.trim().parse::<i64>().expect("right hand values are ints"))
                        .collect::<Vec<_>>(),
                )
            })
        })
        .collect::<Vec<_>>()
}

fn find_res_recursive(deque: &mut VecDeque<i64>) -> (Vec<i64>, Vec<String>) {
    match deque.pop_front() {
        Some(v) => {
            let mut rec = Vec::new();
            let mut repr = Vec::new();
            if deque.is_empty() {
                rec.push(v);
                repr.push(format!("{}", v));
                return (rec, repr);
            }

            let (next_values, repr_vec) = find_res_recursive(deque);

            for next_value in next_values {
                rec.push(v + next_value);
                rec.push(v * next_value);
            }

            for next_value in repr_vec {
                repr.push(format!("{} + {}", v, next_value));
                repr.push(format!("{} * {}", v, next_value));
            }
            (rec, repr)
        }
        None => unreachable!(),
    }
}

fn find_res_recursive_p2(deque: &mut VecDeque<i64>) -> (Vec<i64>, Vec<String>) {
    match deque.pop_front() {
        Some(v) => {
            let mut rec = Vec::new();
            let mut repr = Vec::new();
            if deque.is_empty() {
                rec.push(v);
                repr.push(format!("{}", v));
                return (rec, repr);
            }

            let (next_values, repr_vec) = find_res_recursive_p2(deque);

            for next_value in next_values {
                rec.push(v + next_value);
                rec.push(v * next_value);
                rec.push(
                    format!("{}{}", next_value, v)
                        .parse::<i64>()
                        .expect("concatenation should produce valud number"),
                );
            }

            for next_value in repr_vec {
                repr.push(format!("{} + {}", v, next_value));
                repr.push(format!("{} * {}", v, next_value));
                repr.push(format!("{} || {}", v, next_value));
            }
            (rec, repr)
        }
        None => unreachable!(),
    }
}

impl Solution<i64> for Day7 {
    fn part1(&self, _input: InputData) -> Option<i64> {
        let data = parse_data(_input);

        let mut total_res = 0;
        for (res, elemts) in data {
            // println!("{}: {:?}", res, elemts);

            let mut deque = VecDeque::from(elemts.iter().rev().cloned().collect::<Vec<_>>());
            let (res_vec, _) = find_res_recursive(&mut deque);
            if let Some((_, _)) = res_vec.iter().find_position(|e| **e == res) {
                // println!("=====> Yes: {}", res_repr.get(ri).unwrap());
                total_res += res;
            }
            // else {
            //     println!("======> NO :(");
            //     println!("{:?}", res_repr);
            //     println!("{:?}", res_vec)
            // }
        }
        Some(total_res)
    }

    fn part2(&self, _input: InputData) -> Option<i64> {
        let data = parse_data(_input);

        let mut total_res = 0;
        for (res, elemts) in data {
            // println!("{}: {:?}", res, elemts);

            let mut deque = VecDeque::from(elemts.iter().rev().cloned().collect::<Vec<_>>());
            let (res_vec, _) = find_res_recursive_p2(&mut deque);
            if let Some((_, _)) = res_vec.iter().find_position(|e| **e == res) {
                // println!("=====> Yes: {}", res_repr.get(ri).unwrap());
                total_res += res;
            }
            // else {
            //     println!("======> NO :(");
            //     println!("{:?}", res_repr);
            //     println!("{:?}", res_vec)
            // }
        }
        Some(total_res)
    }

    fn day() -> &'static str {
        DAY
    }
}

fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info)?;

    start_day(DAY);

    let solution = Day7 {};

    solution.run_tests_part1(TEST, 3749);

    solution.run_and_assert_part_1(2654749936343)?;

    solution.run_tests_part2(TEST, 11387);

    solution.run_part_2()?;
    // solution.run_part_2()?;

    Ok(())
}

#[cfg(test)]
mod test {

    // tests
}
