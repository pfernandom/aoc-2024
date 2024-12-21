use std::collections::{HashSet, VecDeque};

use adv_code_2024::*;
use anyhow::*;
use crossterm::style::Stylize;
use log::LevelFilter;
use map::Map2D;
use ndarray::Array2;
const DAY: &str = "10";

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

struct Day10;

macro_rules! vec_deque {
    () => (
        std::collections::VecDeque::new()
    );
    ($elem:expr; $n:expr) => (
        std::collections::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        std::collections::VecDeque::from(
            <[_]>::into_vec(
                // This rustc_box is not required, but it produces a dramatic improvement in compile
                // time when constructing arrays with many elements.
                std::boxed::Box::new([$($x),+])
            )
        )
    );
}

fn navigate(
    map: &mut Map2D,
    pos: (usize, usize),
    v: u32,
    visited: &mut HashSet<(usize, usize)>,
    visited_goals: &mut HashSet<(usize, usize)>,
) -> Vec<VecDeque<(usize, usize, u32)>> {
    // print!("(v={},{:?}),", v, pos);
    if v == 9 {
        visited_goals.insert((pos.0, pos.1));
        vec![vec_deque![(pos.0, pos.1, v)]]
    } else {
        let mut found = Vec::new();
        for coord in map.get_next_possible_hv(&pos) {
            if !visited.contains(&coord) {
                visited.insert(coord);

                if let Some(nv) = map.get(coord) {
                    if *nv == v + 1 {
                        let next_elemets = navigate(map, coord, *nv, visited, visited_goals);

                        for ne in next_elemets {
                            // println!("Inserting path");
                            let mut ne = ne.clone();
                            ne.push_front((pos.0, pos.1, v));
                            found.push(ne);
                        }
                    }
                }

                visited.remove(&coord);
            }
        }

        found
    }
}

impl Solution<i32> for Day10 {
    fn part1(&self, _input: InputData) -> Option<i32> {
        let mut m = Map2D::from_input(_input);
        let mut res = 0;

        for r in 0..m.rows() {
            for c in 0..m.cols() {
                if let Some(0) = m.get((r, c)) {
                    let mut visited = HashSet::new();
                    let mut visited_goals = HashSet::new();
                    visited.insert((r, c));
                    navigate(&mut m, (r, c), 0, &mut visited, &mut visited_goals);

                    res += visited_goals.len();
                }
            }
        }

        Some(res as i32)
    }

    fn part2(&self, _input: InputData) -> Option<i32> {
        let mut m = Map2D::from_input(_input);
        let mut res = 0;

        for r in 0..m.rows() {
            for c in 0..m.cols() {
                if let Some(0) = m.get((r, c)) {
                    let mut visited = HashSet::new();
                    let mut visited_goals = HashSet::new();
                    visited.insert((r, c));
                    let paths_found = navigate(&mut m, (r, c), 0, &mut visited, &mut visited_goals);

                    res += paths_found.len();
                }
            }
        }

        Some(res as i32)
    }

    fn day() -> &'static str {
        DAY
    }
}

fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info)?;

    start_day(DAY);

    let solution = Day10 {};

    solution.run_tests_part1(TEST, 36);

    solution.run_part_1()?;

    solution.run_tests_part2(TEST, 81);

    solution.run_part_2()?;
    // solution.run_part_2()?;

    Ok(())
}

#[cfg(test)]
mod test {

    // tests
}
