use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use adv_code_2024::*;
use anyhow::*;

use log::LevelFilter;
const DAY: &str = "04";

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

struct Day4;

#[derive(Debug)]
struct Matrix {
    data: ndarray::Array2<char>,
    rows: usize,
    columns: usize,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.columns {
                write!(f, "{}", self.data.get((r, c)).unwrap())?
            }
            write!(f, "\n")?
        }

        std::result::Result::Ok(())
    }
}

impl Matrix {
    fn new(vecs: InputData) -> Matrix {
        let arr = parse_char_matrix(vecs.lines().collect::<Vec<_>>());
        Matrix {
            rows: arr.dim().0,
            columns: arr.dim().1,
            data: arr,
        }
    }

    fn get(&self, row: isize, col: isize) -> Option<&char> {
        let rows = self.data.nrows();
        let cols = self.data.ncols();
        if row < 0 || col < 0 || row >= rows as isize || col >= cols as isize {
            return None;
        }

        self.data.get((row as usize, col as usize))
    }

    fn subwords<'a>(&self, start_row: usize, start_col: usize, length: usize) -> Vec<String> {
        let mut substrings = Vec::new();

        // Function to collect characters along a direction
        let collect = |row_step: isize, col_step: isize| -> String {
            let mut result = String::new();
            let mut row = start_row as isize;
            let mut col = start_col as isize;

            for _ in 0..length {
                if let Some(c) = self.get(row, col) {
                    result.push(*c);
                }

                row += row_step;
                col += col_step;
            }
            result
        };

        for row_step in [-1, 0, 1] {
            for col_step in [-1, 0, 1] {
                let s = collect(row_step, col_step); // Right
                if !s.is_empty() {
                    substrings.push(s);
                }
            }
        }

        substrings
    }
}

fn count_strings(strings: Vec<String>) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for s in strings {
        *counts.entry(s).or_insert(0) += 1;
    }

    counts
}

#[macro_export]
macro_rules! concat {
    ($($x:expr),*) => (
        [$($x),*].iter()
        .filter_map(|o| o.clone())
        .collect::<String>()
    );
}

impl Solution<i32> for Day4 {
    fn part1(&self, _input: InputData) -> Option<i32> {
        let mut res: i32 = 0;
        let m = Matrix::new(_input);
        let mut visited = HashSet::new();
        for r in 0..m.rows {
            for c in 0..m.columns {
                if visited.contains(&(r, c)) {
                    continue;
                }
                match m.data.get((r, c)) {
                    Some('X') => {
                        visited.insert((r, c));
                        let v = count_strings(m.subwords(r, c, 4));
                        let counts = v.get("XMAS").unwrap_or(&0);
                        println!("{:?}", counts);
                        res += *counts as i32;
                    }
                    _ => {}
                }
            }
        }

        println!("{}", m);

        Some(res)
    }

    fn part2(&self, _input: InputData) -> Option<i32> {
        let mut res: i32 = 0;
        let m = Matrix::new(_input);
        let mut visited = HashSet::new();

        for r in 0..m.rows as isize {
            for c in 0..m.columns as isize {
                if visited.contains(&(r, c)) {
                    continue;
                }
                match m.get(r, c) {
                    Some('A') => {
                        visited.insert((r, c));

                        let w1 = concat![m.get(r - 1, c - 1), m.get(r, c), m.get(r + 1, c + 1)];
                        let w2 = concat![m.get(r - 1, c + 1), m.get(r, c), m.get(r + 1, c - 1)];

                        if (w1 == "SAM" || w1 == "MAS") && (w2 == "SAM" || w2 == "MAS") {
                            res += 1;
                        } else {
                            println!("Not found at {},{}", r, c);
                        }
                    }
                    _ => {}
                }
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

    let solution = Day4 {};

    solution.run_tests_part1(TEST, 18);

    solution.run_part_1()?;

    solution.run_tests_part2(TEST, 9);

    solution.run_part_2()?;

    Ok(())
}

#[cfg(test)]
mod test {

    // tests
}
