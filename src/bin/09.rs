use std::collections::HashMap;

use adv_code_2024::*;
use anyhow::*;
use itertools::Itertools;
use log::LevelFilter;
const DAY: &str = "09";

const TEST: &str = "\
2333133121414131402
";

struct Day9;

fn get_indices(groups: &Vec<Vec<i32>>) -> HashMap<i32, usize> {
    let mut group_index = HashMap::new();
    for (ind, g) in groups.iter().enumerate() {
        if let Some(v) = g.first() {
            if *v >= 0 {
                group_index.insert(*g.first().unwrap(), ind);
            }
        }
    }
    group_index
}

impl Solution<i64> for Day9 {
    fn part1(&self, _input: InputData) -> Option<i64> {
        let mut r = 0;
        for line in _input.lines() {
            let moved_line = de_compress_and_move(line.trim());

            r += moved_line
                .iter()
                .enumerate()
                .filter_map(|(i, c)| c.map(|v| v * i as i64))
                .reduce(|acc, e| acc + e)
                .unwrap_or(0);
        }

        Some(r)
    }

    fn part2(&self, _input: InputData) -> Option<i64> {
        let mut r = 0;
        for line in _input.lines() {
            let mut i = 0;
            let mut is_space = false;
            let mut res = Vec::new();
            let mut groups = Vec::new();

            for c in line.chars() {
                if is_space {
                    let size = c.to_digit(10).expect("should be digit");
                    if size == 0 {
                        is_space = !is_space;
                        continue;
                    }
                    let mut tmp = Vec::new();
                    for _ in 0..size {
                        res.push(None);
                        tmp.push(-1);
                    }
                    groups.push(tmp);
                } else {
                    let size = c.to_digit(10).expect("should be digit");
                    if size == 0 {
                        is_space = !is_space;
                        continue;
                    }
                    let mut tmp = Vec::new();
                    for _ in 0..size {
                        res.push(Some(i));
                        tmp.push(i);
                    }

                    groups.push(tmp);
                    i += 1
                }
                is_space = !is_space;
            }

            let max_i = i;

            // println!(
            //     "{}",
            //     res.iter()
            //         .map(|e| e.map(|e| format!("{}", e)).unwrap_or(String::from(".")))
            //         .join("")
            // );

            let mut group_index = get_indices(&groups);

            // println!("groups: {:?}", groups);
            // println!("group_index: {:?}", group_index);

            for i in (0..max_i).rev() {
                let group_ind = group_index.get(&i).unwrap();
                // println!("{}:{:?}", i, group_index.get(&i));

                let cur_elems = groups.get(*group_ind).unwrap().clone();

                if let Some((first_empty_index, first_empty_froup)) = groups
                    .iter()
                    .find_position(|l| l.contains(&-1) && l.len() >= cur_elems.len())
                    .map(|(i, p)| (i, p.clone()))
                {
                    if first_empty_index >= *group_ind {
                        continue;
                    }

                    groups[first_empty_index] = cur_elems.clone();
                    groups[*group_ind] = vec![-1; cur_elems.len()];
                    if first_empty_froup.len() >= cur_elems.len() {
                        groups.insert(
                            first_empty_index + 1,
                            vec![-1; first_empty_froup.len() - cur_elems.len()],
                        );
                    }
                }

                // println!(
                //     "new groups: {:?}",
                //     groups
                //         .iter()
                //         .flat_map(|g| g.iter().map(|c| if *c >= 0 {
                //             format!("{}", c)
                //         } else {
                //             String::from(".")
                //         }))
                //         .join("")
                // );

                group_index = get_indices(&groups);
            }

            r += groups
                .iter()
                .flatten()
                .enumerate()
                .filter(|(_, c)| **c >= 0)
                .map(|(i, e)| i as i64 * *e as i64)
                .reduce(|acc, el| el + acc)
                .unwrap_or(0);
        }

        Some(r)
    }

    fn day() -> &'static str {
        DAY
    }
}

fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info)?;

    start_day(DAY);

    let solution = Day9 {};

    solution.run_tests_part1(TEST, 1928);

    solution.run_and_assert_part_1(6310675819476)?;

    solution.run_tests_part2(TEST, 2858);

    solution.run_part_2()?;
    // solution.run_part_2()?;

    Ok(())
}

fn de_compress<T: AsRef<str>>(line: T) -> Vec<Option<i64>> {
    let mut i = 0;
    let mut is_space = false;
    let mut res = Vec::new();
    for c in line.as_ref().chars() {
        if is_space {
            for _ in 0..c.to_digit(10).expect("should be digit") {
                res.push(None);
            }
        } else {
            for _ in 0..c.to_digit(10).expect("should be digit") {
                res.push(Some(i));
            }
            i += 1
        }
        is_space = !is_space;
    }

    res
}

fn de_compress_and_move<T: AsRef<str>>(line: T) -> Vec<Option<i64>> {
    let decompressed = de_compress(line);

    let mut dec_chars = decompressed.clone();

    let empty_elements = decompressed.iter().enumerate().filter(|c| c.1.is_none());

    let non_empty_elements = decompressed
        .iter()
        .enumerate()
        .filter(|c| c.1.is_some())
        .collect::<Vec<_>>();

    let non_empty_elements = non_empty_elements.iter().rev();

    for (empty, non_empty) in empty_elements.zip(non_empty_elements) {
        // println!("{:?} and {:?} in {:?}", empty, non_empty, dec_chars);

        if dec_chars[empty.0].is_none() && dec_chars[non_empty.0].is_some() && empty.0 < non_empty.0
        {
            dec_chars[empty.0] = non_empty.1.clone();
            dec_chars[non_empty.0] = None;
        }

        // println!("{}", dec_chars.iter().map(|e| e.unwrap_or('.')).join(""));
    }

    dec_chars
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::{de_compress, de_compress_and_move, get_indices};

    #[test]
    fn test_decompress_v2() {
        assert_eq!(
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ],
            de_compress("12345")
        )
    }

    #[test]
    fn teststuff() {
        let line = "2333133121414131402";

        let mut i = 0;
        let mut is_space = false;
        let mut res = Vec::new();
        let mut groups = Vec::new();

        for c in line.chars() {
            if is_space {
                let size = c.to_digit(10).expect("should be digit");
                if size == 0 {
                    is_space = !is_space;
                    continue;
                }
                let mut tmp = Vec::new();
                for _ in 0..size {
                    res.push(None);
                    tmp.push(-1);
                }
                groups.push(tmp);
            } else {
                let size = c.to_digit(10).expect("should be digit");
                if size == 0 {
                    is_space = !is_space;
                    continue;
                }
                let mut tmp = Vec::new();
                for _ in 0..size {
                    res.push(Some(i));
                    tmp.push(i);
                }

                groups.push(tmp);
                i += 1
            }
            is_space = !is_space;
        }

        let max_i = i;

        println!(
            "{}",
            res.iter()
                .map(|e| e.map(|e| format!("{}", e)).unwrap_or(String::from(".")))
                .join("")
        );

        let mut group_index = get_indices(&groups);

        // println!("groups: {:?}", groups);
        // println!("group_index: {:?}", group_index);

        for i in (0..max_i).rev() {
            let group_ind = group_index.get(&i).unwrap();
            // println!("{}:{:?}", i, group_index.get(&i));

            let cur_elems = groups.get(*group_ind).unwrap().clone();

            if let Some((first_empty_index, first_empty_froup)) = groups
                .iter()
                .find_position(|l| l.contains(&-1) && l.len() >= cur_elems.len())
                .map(|(i, p)| (i, p.clone()))
            {
                if first_empty_index >= *group_ind {
                    continue;
                }

                groups[first_empty_index] = cur_elems.clone();
                groups[*group_ind] = vec![-1; cur_elems.len()];
                if first_empty_froup.len() >= cur_elems.len() {
                    groups.insert(
                        first_empty_index + 1,
                        vec![-1; first_empty_froup.len() - cur_elems.len()],
                    );
                }
            }

            println!(
                "new groups: {:?}",
                groups
                    .iter()
                    .flat_map(|g| g.iter().map(|c| if *c >= 0 {
                        format!("{}", c)
                    } else {
                        String::from(".")
                    }))
                    .join("")
            );

            group_index = get_indices(&groups);
        }

        groups
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, e)| i as i64 * *e as i64)
            .reduce(|acc, el| el + acc);
    }

    #[test]
    fn test_decompress_and_move_v2() {
        assert_eq!(
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            de_compress_and_move("12345")
        )
    }

    // tests
}
