use std::collections::{HashMap, HashSet};

use adv_code_2024::*;
use anyhow::*;
use log::LevelFilter;
const DAY: &str = "08";

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

struct Day8;

impl Solution<i32> for Day8 {
    fn part1(&self, _input: InputData) -> Option<i32> {
        let mut m = parse_char_matrix(_input.lines().collect::<Vec<_>>());

        let mut nodes = HashMap::new();
        for r in 0..m.dim().0 {
            for c in 0..m.dim().1 {
                match m.get((r, c)) {
                    Some(v) => {
                        if *v != '.' {
                            nodes.entry(*v).or_insert(Vec::new()).push((r, c));
                        }
                    }
                    None => todo!(),
                }
            }
        }

        // let mut antinodes = HashMap::new();
        let mut all_antinodes = HashSet::new();

        for (node_type, mut towers) in nodes {
            towers.sort();

            let rows = m.dim().0;
            let cols = m.dim().1;

            for j in 0..towers.len() {
                for i in 0..j {
                    let p1 = towers.get(i).unwrap();
                    let p2 = towers.get(j).unwrap();

                    for p in get_antinodes(p1, p2) {
                        if p.0 < rows && p.1 < cols && m[[p.0, p.1]] != node_type {
                            m[[p.0, p.1]] = '#';
                            all_antinodes.insert(p);
                        }
                    }
                }
            }
        }
        // println!("{:?}", m);
        Some(all_antinodes.len() as i32)
    }

    fn part2(&self, _input: InputData) -> Option<i32> {
        let mut m = parse_char_matrix(_input.lines().collect::<Vec<_>>());

        let mut nodes = HashMap::new();
        for r in 0..m.dim().0 {
            for c in 0..m.dim().1 {
                match m.get((r, c)) {
                    Some(v) => {
                        if *v != '.' {
                            nodes.entry(*v).or_insert(Vec::new()).push((r, c));
                        }
                    }
                    None => todo!(),
                }
            }
        }

        let rows = m.dim().0;
        let cols = m.dim().1;

        for (node_type, mut towers) in nodes {
            towers.sort();

            for j in 0..towers.len() {
                for i in 0..j {
                    let p1 = towers.get(i).unwrap();
                    let p2 = towers.get(j).unwrap();

                    for p in get_antinodes_p2(rows, cols, p1, p2) {
                        if m[[p.0, p.1]] == node_type || m[[p.0, p.1]] == '.' {
                            m[[p.0, p.1]] = '#';

                            // all_antinodes.insert(p);
                        }
                        // }
                    }
                }
            }
        }
        println!("{:?}", m);

        let mut res = 0;
        for r in 0..rows {
            for c in 0..cols {
                if m[[r, c]] != '.' {
                    res += 1;
                }
            }
        }
        Some(res)
    }

    fn day() -> &'static str {
        DAY
    }
}

/**
 * 2654749936343 - too high
 */
fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info)?;

    start_day(DAY);

    let solution = Day8 {};

    solution.run_tests_part1(TEST, 14);

    // solution.run_part_1()?;
    solution.run_and_assert_part_1(369)?;

    solution.run_tests_part2(TEST, 34);

    solution.run_part_2()?;
    // solution.run_part_2()?;

    Ok(())
}

fn get_antinodes(p1: &(usize, usize), p2: &(usize, usize)) -> Vec<(usize, usize)> {
    let dist_r = p2.0 as isize - p1.0 as isize;
    let dist_c = p2.1 as isize - p1.1 as isize;

    let mut res: Vec<(usize, usize)> = Vec::new();

    if let Some(p1) = Some((p1.0 as isize - dist_r, p1.1 as isize - dist_c))
        .take_if(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(v1, v2)| (v1 as usize, v2 as usize))
    {
        res.push(p1);
    }

    if let Some(p2) = Some((p2.0 as isize + dist_r, p2.1 as isize + dist_c))
        .take_if(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(v1, v2)| (v1 as usize, v2 as usize))
    {
        res.push(p2);
    }

    res
}

fn get_antinodes_p2(
    rows: usize,
    cols: usize,
    p1: &(usize, usize),
    p2: &(usize, usize),
) -> Vec<(usize, usize)> {
    let dist_r = p2.0 as isize - p1.0 as isize;
    let dist_c = p2.1 as isize - p1.1 as isize;

    let mut res: Vec<(usize, usize)> = Vec::new();

    let accumulate_back = |point: (usize, usize)| -> Vec<(isize, isize)> {
        let mut point = (point.0 as isize, point.1 as isize);
        let mut points = Vec::new();
        while point.0 > 0 && point.1 > 0 {
            point = (point.0 - dist_r, point.1 - dist_c);
            points.push(point);
        }
        points
    };

    let accumulate_fw = |point: (usize, usize)| -> Vec<(isize, isize)> {
        let mut point = (point.0 as isize, point.1 as isize);
        let mut points = Vec::new();
        while point.0 < rows as isize && point.1 < cols as isize {
            point = (point.0 + dist_r, point.1 + dist_c);
            points.push(point);
        }
        points
    };

    for p1 in accumulate_back(*p1)
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < rows as isize && *y < cols as isize)
        .map(|(v1, v2)| (*v1 as usize, *v2 as usize))
    {
        res.push(p1);
    }

    for p2 in accumulate_fw(*p2)
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < rows as isize && *y < cols as isize)
        .map(|(v1, v2)| (*v1 as usize, *v2 as usize))
    {
        res.push(p2);
    }

    res
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use adv_code_2024::parse_char_matrix;

    use crate::get_antinodes;

    // use crate::get_antinodes;

    #[test]
    fn test_get_antinodes() {
        let inp = "\
        ..........
        ..........
        ..........
        ....a.....
        ..........
        .....a....
        ..........
        ..........
        ..........
        ..........
            ";
        let mut m = parse_char_matrix(
            inp.lines()
                .map(|l| l.trim())
                .filter(|l| l.len() > 0)
                .collect::<Vec<_>>(),
        );

        let mut nodes = HashMap::new();
        for r in 0..m.dim().0 {
            for c in 0..m.dim().1 {
                match m.get((r, c)) {
                    Some(v) => {
                        if *v != '.' {
                            nodes.entry(*v).or_insert(Vec::new()).push((r, c));
                        }
                    }
                    None => todo!(),
                }
            }
        }

        println!("nodes:{:?}", nodes);
        let mut towers = nodes.get(&'a').unwrap().clone();
        towers.sort();

        let p1 = nodes.get(&'a').unwrap().get(0).unwrap();
        let p2 = nodes.get(&'a').unwrap().get(1).unwrap();

        for p in get_antinodes(p1, p2) {
            m[[p.0, p.1]] = '#';
        }

        println!("{:?}", m);
    }

    #[test]
    fn test_get_antinodes_2() {
        let inp = "\
        ..........
        ..........
        ..........
        .....a....
        ..........
        ....a.....
        ..........
        ..........
        ..........
        ..........
            ";
        let mut m = parse_char_matrix(
            inp.lines()
                .map(|l| l.trim())
                .filter(|l| l.len() > 0)
                .collect::<Vec<_>>(),
        );

        let mut nodes = HashMap::new();
        for r in 0..m.dim().0 {
            for c in 0..m.dim().1 {
                match m.get((r, c)) {
                    Some(v) => {
                        if *v != '.' {
                            nodes.entry(*v).or_insert(Vec::new()).push((r, c));
                        }
                    }
                    None => todo!(),
                }
            }
        }

        println!("nodes:{:?}", nodes);
        let mut towers = nodes.get(&'a').unwrap().clone();
        towers.sort();

        let p1 = nodes.get(&'a').unwrap().get(0).unwrap();
        let p2 = nodes.get(&'a').unwrap().get(1).unwrap();

        for p in get_antinodes(p1, p2) {
            m[[p.0, p.1]] = '#';
        }

        println!("{:?}", m);
    }

    #[test]
    fn test_get_antinodes_3() {
        let inp = "\
        ..........
        ..........
        ..........
        ....a.....
        ........a.
        .....a....
        ..........
        ..........
        ..........
        ..........";
        let mut m = parse_char_matrix(
            inp.lines()
                .map(|l| l.trim())
                .filter(|l| l.len() > 0)
                .collect::<Vec<_>>(),
        );

        let mut nodes = HashMap::new();
        for r in 0..m.dim().0 {
            for c in 0..m.dim().1 {
                match m.get((r, c)) {
                    Some(v) => {
                        if *v != '.' {
                            nodes.entry(*v).or_insert(Vec::new()).push((r, c));
                        }
                    }
                    None => todo!(),
                }
            }
        }

        println!("nodes:{:?}", nodes);
        let mut towers = nodes.get(&'a').unwrap().clone();
        towers.sort();

        let rows = m.dim().0;
        let cols = m.dim().1;

        for j in 0..towers.len() {
            for i in 0..j {
                let p1 = towers.get(i).unwrap();
                let p2 = towers.get(j).unwrap();

                println!("towers: {:?} and {:?}", p1, p2);

                for p in get_antinodes(p1, p2) {
                    if p.0 < rows && p.1 < cols {
                        m[[p.0, p.1]] = '#';
                    }
                }
            }
        }

        println!("{:?}", m);
    }
    // tests
}
