use std::{cmp::Ordering, collections::HashSet};

use adv_code_2024::*;
use anyhow::*;
use log::LevelFilter;
const DAY: &str = "05";

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

struct Day2;

impl Solution<i32> for Day2 {
    fn part1(&self, input: InputData) -> Option<i32> {
        let string_not_empty = |s: &&String| -> bool { s.trim().len() > 0 };

        let d1 = input
            .lines()
            .take_while(string_not_empty)
            .filter_map(|l| l.split_once("|"))
            .collect::<HashSet<_>>();

        let d2 = input
            .lines()
            .skip_while(string_not_empty)
            .skip(1)
            .take_while(string_not_empty)
            .map(|l| l.split(",").collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut res = 0;

        for mut row in d2 {
            let before = row.join("");
            row.sort_by(|a, b| {
                if d1.contains(&((a, b))) {
                    Ordering::Less
                } else if d1.contains(&((b, a))) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            let after = row.join("");

            println!("======");

            if before.eq(&after) {
                println!("Matching element: {:?} {:?}", before, after);
                let mid = row.len() / 2;
                let mid_element = row.get(mid).unwrap().parse::<i32>().unwrap();

                res += mid_element;
            } else {
                println!("Bad ordering: {:?} and {:?}", before, after)
            }
        }

        Some(res)
    }

    fn part2(&self, input: InputData) -> Option<i32> {
        let string_not_empty = |s: &&String| -> bool { s.trim().len() > 0 };

        let d1 = input
            .lines()
            .take_while(string_not_empty)
            .filter_map(|l| l.split_once("|"))
            .collect::<HashSet<_>>();

        let d2 = input
            .lines()
            .skip_while(string_not_empty)
            .skip(1)
            .take_while(string_not_empty)
            .map(|l| l.split(",").collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut res = 0;

        for mut row in d2 {
            let before = row.join("");
            row.sort_by(|a, b| {
                if d1.contains(&((a, b))) {
                    Ordering::Less
                } else if d1.contains(&((b, a))) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            let after = row.join("");

            if !before.eq(&after) {
                let mid = row.len() / 2;
                let mid_element = row.get(mid).unwrap().parse::<i32>().unwrap();

                res += mid_element;
            }
        }

        Some(res)
    }

    fn day() -> &'static str {
        DAY
    }
}

fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info)?;

    start_day(DAY);

    let solution = Day2 {};

    solution.run_tests_part1(TEST, 143);

    solution.run_part_1()?;

    solution.run_tests_part2(TEST, 123);

    solution.run_part_2()?;
    // solution.run_part_2()?;

    Ok(())
}

#[cfg(test)]
mod test {

    // tests
}
