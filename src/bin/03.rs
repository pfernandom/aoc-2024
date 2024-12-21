use adv_code_2024::*;
use anyhow::*;
use log::LevelFilter;
use regex::Regex;
use std::sync::OnceLock;

const DAY: &str = "03";

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

/**
 * 102210215 - too high
 * 12882998 - too low
 *
 */
fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info)?;

    start_day(DAY);

    let solution = Day3 {};

    solution.run_tests_part1(TEST, 161);

    solution.run_part_1()?;

    solution.run_tests_part2(TEST2, 48);

    solution.run_part_2()?;
    // solution.run_part_2()?;

    Ok(())
}

struct Day3;

#[derive(Debug, PartialEq, Eq)]
enum MulMatch {
    Do(usize),
    Dont(usize),
    Match(usize, i32, i32),
}

trait Locable {
    fn get_start(&self) -> usize;
}

impl Locable for MulMatch {
    fn get_start(&self) -> usize {
        match self {
            MulMatch::Do(s) => *s,
            MulMatch::Dont(s) => *s,
            MulMatch::Match(s, _, _) => *s,
        }
    }
}

impl PartialOrd for MulMatch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_start().partial_cmp(&other.get_start())
    }
}

impl Ord for MulMatch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_start().cmp(&other.get_start())
    }
}

impl Solution<i32> for Day3 {
    fn part1(&self, input: InputData) -> Option<i32> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut res = 0;
        for line in input.lines() {
            let captures = re.captures_iter(&line);

            for (_, [n1, n2]) in captures.map(|c| c.extract()) {
                let d1: i32 = n1.parse().unwrap();
                let d2: i32 = n2.parse().unwrap();
                res += d1 * d2;
            }
        }
        Some(res)
    }

    fn part2(&self, input: InputData) -> Option<i32> {
        let mut res = 0;
        for line in input.lines() {
            let control_matches = find_matches(line);

            let (_, line_res) =
                control_matches
                    .iter()
                    .fold((true, 0), |(is_enabled, res), mtch| match mtch {
                        MulMatch::Do(_) => (true, res),
                        MulMatch::Dont(_) => (false, res),
                        MulMatch::Match(_, n1, n2) => {
                            if is_enabled {
                                (true, res + (n1 * n2))
                            } else {
                                (false, res)
                            }
                        }
                    });

            res += line_res;
        }
        Some(res)
    }

    fn day() -> &'static str {
        DAY
    }
}

fn find_matches<S: AsRef<str>>(input: S) -> Vec<MulMatch> {
    let line = input.as_ref();
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

    let captures = re.captures_iter(&line);

    let mut control_matches = vec![];

    line.match_indices("do()")
        .for_each(|m| control_matches.push(MulMatch::Do(m.0)));
    line.match_indices("don't()")
        .for_each(|m| control_matches.push(MulMatch::Dont(m.0)));

    captures.for_each(|capture| {
        let mut it = capture.iter();
        let (_, [n1, n2]) = capture.extract();
        let l = it.next().flatten().expect("should have group");
        control_matches.push(MulMatch::Match(
            l.start(),
            n1.parse().unwrap(),
            n2.parse().unwrap(),
        ));
    });

    control_matches.sort();

    // println!("control_matches: {:?}", control_matches);

    control_matches
}

#[cfg(test)]
mod test {
    use crate::{find_matches, Day3, MulMatch, DAY};
    use adv_code_2024::{read_input, InputData, Solution};
    use colored::Colorize;

    #[test]
    fn test_p2() {
        let solution = Day3 {};
        for (line, expected) in [
            (
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                48,
            ),
            (
                "don't()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                40,
            ),
            (
                "don't()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()don't()?mul(8,5))",
                0
            ),
            (
                "mul(100,2)",
                200,
            ),
            (
                "mul(1000,2)",
                0,
            ),
            (
                "ddo()mul(2,2)",
                4,
            ),
            (
                "do(())mul(2,2)",
                4,
            ),
            (
                "don't(())mul(2,2)",
                4,
            ),
        ]
        .iter()
        .map(|(line, expected)| (*line, expected))
        {
            let res = solution.part2(InputData::from_str(line));

            assert_eq!(Some(*expected), res, "expected: {:?}, actual: {:?}", Some(*expected), res)
        }
    }

    #[test]
    fn find_matches_test() {
        let inp = read_input(DAY).unwrap();
        let mut sum = 0;
        let mut enabled = true;
        for line in inp.lines() {
            let res = find_matches(line.clone());

            for m in &res {
                match *m {
                    MulMatch::Do(i) => {
                        assert_eq!(Some("do()"), line.get(i..i + "do()".len()));
                        enabled = true;
                    }
                    MulMatch::Dont(i) => {
                        assert_eq!(Some("don't()"), line.get(i..i + "don't()".len()));
                        enabled = false;
                    }
                    MulMatch::Match(i, n1, n2) => {
                        let maybe = format!("mul({n1},{n2})");
                        assert_eq!(Some(maybe.as_str()), line.get(i..i + maybe.len()));
                        if enabled {
                            sum += n1 * n2;
                        }
                    }
                };
            }

            let mut ind = 0;
            let mut new_line = String::new();

            {
                let mut push_to_line = |s: usize, e: usize, (r, g, b): (u8, u8, u8)| {
                    let prefix = line.get(ind..s).unwrap();
                    let word = line.get(s..e).unwrap();

                    new_line.push_str(prefix);
                    new_line.push_str(word.truecolor(r, g, b).to_string().as_str());
                    ind = e;
                };

                for m in res {
                    match m {
                        MulMatch::Do(i) => {
                            let s = i;
                            let e = i + "do()".len();

                            push_to_line(s, e, (0, 255, 0));
                            assert_eq!(Some("do()"), line.get(i..i + "do()".len()))
                        }
                        MulMatch::Dont(i) => {
                            let s = i;
                            let e = i + "don't()".len();

                            push_to_line(s, e, (255, 0, 0));
                            assert_eq!(Some("don't()"), line.get(i..i + "don't()".len()))
                        }
                        MulMatch::Match(i, n1, n2) => {
                            let maybe = format!("mul({n1},{n2})");
                            let s = i;
                            let e = i + maybe.len();

                            push_to_line(s, e, (0, 0, 255));
                            assert_eq!(Some(maybe.as_str()), line.get(i..i + maybe.len()))
                        }
                    };
                }

                new_line.push_str(line.get(ind..line.len()).unwrap());

                println!("==== diff =====");
                println!("{}", new_line);
                println!("{}", line);
            }
            println!("R={}", sum)
        }
    }

    // tests
}
