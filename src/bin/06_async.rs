use adv_code_2024::*;
use anyhow::*;
use async_trait::async_trait;
use crossterm::style::Stylize;
use std::{collections::HashSet, fmt::Display};
use tokio::sync::mpsc;
// use itertools::Itertools;
use log::LevelFilter;
use ndarray::Array2;
const DAY: &str = "06";

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

struct Day2;

#[derive(Clone)]
struct Map {
    data: Array2<char>,
    guard: Option<(usize, usize)>,
    guard_v2: Guard,
    path_len: usize,
}

#[derive(Debug, PartialEq)]
enum State {
    Done,
    NotDone,
    Looped,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                if let Some(content) = self.data.get((r, c)).map(|c| match *c {
                    'O' => format!("{}", *c).blue(),
                    '#' => format!("{}", *c).red(),
                    '|' | '-' => format!("{}", *c).white(),
                    '^' | '>' | '<' | 'v' => format!("{}", *c).green(),
                    '.' => format!("{}", *c).grey(),
                    'H' => format!("{}", *c).black().on_white(),
                    _ => format!("{}", *c).white(),
                }) {
                    write!(f, "{}", content)?
                }
            }
            write!(f, "\n")?
        }

        std::result::Result::Ok(())
    }
}

#[derive(Clone, Debug)]
struct Guard {
    pos: (usize, usize),
    val: GuardDirection,
    history: Vec<((usize, usize), GuardDirection)>,
    visited: HashSet<(usize, usize, char)>,
}

#[derive(Debug, Clone)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GuardDirection {
    fn from_char(c: char) -> GuardDirection {
        match c {
            '^' => GuardDirection::Up,
            '>' => GuardDirection::Right,
            'v' => GuardDirection::Down,
            '<' => GuardDirection::Left,
            _ => panic!("unexpected direction {}", c),
        }
    }
    fn to_char(&self) -> char {
        match self {
            GuardDirection::Up => '^',
            GuardDirection::Down => 'v',
            GuardDirection::Left => '<',
            GuardDirection::Right => '>',
        }
    }

    fn next_move(&self, cur_pos: (usize, usize)) -> (isize, isize) {
        let (mr, mc) = match &self {
            GuardDirection::Up => (-1, 0),
            GuardDirection::Down => (1, 0),
            GuardDirection::Left => (0, -1),
            GuardDirection::Right => (0, 1),
        };

        return (cur_pos.0 as isize + mr, cur_pos.1 as isize + mc);
    }

    fn next_direction(&self, char_at_next_pos: char) -> GuardDirection {
        match char_at_next_pos {
            '#' => match &self {
                GuardDirection::Up => GuardDirection::Right,
                GuardDirection::Down => GuardDirection::Left,
                GuardDirection::Left => GuardDirection::Up,
                GuardDirection::Right => GuardDirection::Down,
            },
            _ => self.clone(),
        }
    }
}

enum GuardMoveType {
    Advance {
        from: (usize, usize),
        to: (usize, usize),
        found_loop: bool,
    },
    Turn,
}

impl Guard {
    fn new(initial_pos: (usize, usize), initial_val: char) -> Guard {
        return Guard {
            pos: initial_pos,
            val: GuardDirection::from_char(initial_val),
            history: Vec::new(),
            visited: HashSet::new(),
        };
    }

    fn find_next_move(&self, map: &mut Map) -> Option<GuardMoveType> {
        let (nr, nc) = self.val.next_move(self.pos);
        match map.get(nr, nc) {
            Some('#') => Some(GuardMoveType::Turn),
            Some(c) => {
                let old_pos = self.pos;
                Some(GuardMoveType::Advance {
                    from: old_pos,
                    to: self.pos,
                    found_loop: self.val.to_char() == *c,
                })
            }
            None => None,
        }
    }

    fn move_next(&mut self, map: &mut Map) -> Option<GuardMoveType> {
        self.history.push((self.pos, self.val.clone()));

        let (nr, nc) = self.val.next_move(self.pos);
        // println!("old_pos: {:?}, new_pos:{:?}", self.pos, (nr, nc));

        match map.get(nr, nc) {
            Some('#') => {
                self.val = self.val.next_direction('#');
                Some(GuardMoveType::Turn)
            }
            Some(c) => {
                let old_pos = self.pos;
                self.pos = (nr as usize, nc as usize);

                let visited_key = (nr as usize, nc as usize, self.val.to_char());
                if self.visited.contains(&visited_key) {
                    // println!(
                    //     "Found visiting thing!: old_pos:{:?}, cur_pos={:?}, {:?}",
                    //     self.pos, old_pos, visited_key
                    // );
                    Some(GuardMoveType::Advance {
                        from: old_pos,
                        to: self.pos,
                        found_loop: true,
                    })
                } else {
                    self.visited.insert(visited_key);
                    Some(GuardMoveType::Advance {
                        from: old_pos,
                        to: self.pos,
                        found_loop: self.val.to_char() == *c,
                    })
                }
            }
            None => None,
        }
    }
}

enum MoveType {
    Advance {
        from: (usize, usize),
        to: (usize, usize),
    },
    TurnLeft((usize, usize)),
    TurnRight((usize, usize)),
    TurnDown((usize, usize)),
    TurnUp((usize, usize)),
}

// macro_rules! println {
//     ($($rest:tt)*) => {
//         // if std::env::var("DEBUG").is_ok() {
//             std::println!($($rest)*)
//         // }
//     }
// }

impl Map {
    fn from_input_data(input: &InputData) -> Map {
        let m = parse_char_matrix(input.lines().collect::<Vec<_>>());
        let rows = m.dim().0;
        let cols = m.dim().1;

        let get_guard = || {
            for r in 0..rows {
                for c in 0..cols {
                    if m.get((r, c)).filter(|c| **c == '^').is_some() {
                        return Some((r, c));
                    }
                }
            }
            None
        };

        let guard = get_guard();

        Map {
            data: m.clone(),
            guard,
            path_len: 0,
            guard_v2: Guard::new(guard.unwrap(), '^'),
        }
    }

    fn rows(&self) -> usize {
        self.data.dim().0
    }

    fn cols(&self) -> usize {
        self.data.dim().1
    }

    fn is_out_of_bounds(&self, row: isize, col: isize) -> bool {
        row < 0 || col < 0 || row > self.rows() as isize || col > self.cols() as isize
    }

    fn get(&self, row: isize, col: isize) -> Option<&char> {
        if self.is_out_of_bounds(row, col) {
            None
        } else {
            self.data.get((row as usize, col as usize))
        }
    }

    fn get_next_move(&self, guard_post: Option<(usize, usize)>) -> Option<MoveType> {
        guard_post
            .map(|(r, c)| {
                let ir = r as isize;
                let ic = c as isize;

                self.get(r as isize, c as isize)
                    .map(|c| match c {
                        '^' => ((ir - 1, ic), c),
                        '>' => ((ir, ic + 1), c),
                        '<' => ((ir, ic - 1), c),
                        'v' => ((ir + 1, ic), c),
                        _ => panic!("unexpected char: {}", c),
                    })
                    .map(
                        |((next_row, next_col), cur_c)| match self.get(next_row, next_col) {
                            Some('#') => match cur_c {
                                '^' => Some(MoveType::TurnRight((r, c))),
                                '>' => Some(MoveType::TurnDown((r, c))),
                                'v' => Some(MoveType::TurnLeft((r, c))),
                                '<' => Some(MoveType::TurnUp((r, c))),
                                _ => unreachable!(),
                            },
                            Some(_) => Some(MoveType::Advance {
                                from: (r, c),
                                to: (next_row as usize, next_col as usize),
                            }),
                            None => None,
                        },
                    )
                    .flatten()
            })
            .flatten()
    }

    fn tick(&mut self) -> State {
        match self.get_next_move(self.guard) {
            Some(mt) => match mt {
                MoveType::Advance { from, to } => {
                    match self.data[[to.0, to.1]] {
                        '.' => {
                            self.path_len += 1;
                        }
                        _ => {}
                    }

                    self.data[[to.0, to.1]] = self.data[[from.0, from.1]];
                    self.data[[from.0, from.1]] = 'X';
                    self.guard = Some(to);

                    State::NotDone
                }
                MoveType::TurnLeft(pos) => {
                    self.data[[pos.0, pos.1]] = '<';
                    State::NotDone
                }
                MoveType::TurnRight(pos) => {
                    self.data[[pos.0, pos.1]] = '>';
                    State::NotDone
                }
                MoveType::TurnDown(pos) => {
                    self.data[[pos.0, pos.1]] = 'v';
                    State::NotDone
                }
                MoveType::TurnUp(pos) => {
                    self.data[[pos.0, pos.1]] = '^';
                    State::NotDone
                }
            },
            None => State::Done,
        }
    }
}

#[async_trait]
impl AsyncSolution<i32> for Day2 {
    async fn part1(&self, input: InputData) -> Option<i32> {
        let mut map = Map::from_input_data(&input);

        while let State::NotDone = map.tick() {
            // println!("==== tick ====");
            // println!("{}", map);
        }

        Some(map.path_len as i32 + 1)
    }

    async fn part2(&self, input: InputData) -> Option<i32> {
        let (tx, mut rx) = mpsc::channel(1);
        let mut map = Map::from_input_data(&input);

        let mut cur_state = State::NotDone;
        let mut guard = map.guard_v2.clone();

        while let State::NotDone = cur_state {
            let mut guard_ghost = guard.clone();

            cur_state = match guard.move_next(&mut map) {
                Some(move_t) => match move_t {
                    GuardMoveType::Advance {
                        from,
                        to,
                        found_loop,
                    } => {
                        match (map.data[[from.0, from.1]], map.data[[to.0, to.1]]) {
                            ('^', '>') | ('>', 'v') | ('v', '<') | ('<', '^') => {
                                if let Some(GuardMoveType::Advance { to: to_2, .. }) =
                                    guard.find_next_move(&mut map)
                                {
                                    match map.get(to_2.0 as isize, to_2.1 as isize) {
                                        Some('.') => {
                                            let ntx = tx.clone();
                                            let mut local_map = map.clone();
                                            tokio::spawn(async move {
                                                local_map.data[[to_2.0, to_2.1]] = '#';
                                                while let Some(ghost_state) =
                                                    guard_ghost.move_next(&mut local_map)
                                                {
                                                    match ghost_state {
                                                        GuardMoveType::Advance {
                                                            found_loop,
                                                            // to: to_2,
                                                            ..
                                                        } => {
                                                            if found_loop {
                                                                // new_blocks.insert(to);
                                                                if let Err(_) = ntx.send(1).await {
                                                                    println!("receiver dropped");
                                                                    return;
                                                                }
                                                                // map.update_block_space(to, to_2);
                                                                return;
                                                            }
                                                        }
                                                        GuardMoveType::Turn => {}
                                                    }
                                                    local_map.data
                                                        [[guard_ghost.pos.0, guard_ghost.pos.1]] =
                                                        guard_ghost.val.to_char();
                                                }
                                            });
                                        }
                                        _ => {}
                                    };
                                }
                            }
                            (_, '.') => {
                                let ntx = tx.clone();
                                let mut local_map = map.clone();
                                tokio::spawn(async move {
                                    local_map.data[[to.0, to.1]] = '#';
                                    while let Some(ghost_state) =
                                        guard_ghost.move_next(&mut local_map)
                                    {
                                        match ghost_state {
                                            GuardMoveType::Advance { found_loop, .. } => {
                                                // local_map.data[[to.0, to.1]] =
                                                //     guard_ghost.val.to_char();

                                                // println!("to_2:{:?}", to_2);
                                                if found_loop {
                                                    // new_blocks.insert(to);
                                                    // map.update_block_space(to, to_2);
                                                    if let Err(_) = ntx.send(1).await {
                                                        println!("receiver dropped");
                                                        return;
                                                    }
                                                    return;
                                                }
                                            }
                                            GuardMoveType::Turn => {
                                                // println!("TURN")
                                            }
                                        };

                                        local_map.data[[guard_ghost.pos.0, guard_ghost.pos.1]] =
                                            guard_ghost.val.to_char();

                                        // println!("Local map:");
                                        // println!("{}", local_map);
                                        // sleep(Duration::from_millis(500));
                                    }
                                });
                            }
                            _ => {}
                        }

                        map.data[[to.0, to.1]] = guard.val.to_char();

                        if found_loop {
                            State::Looped
                        } else {
                            State::NotDone
                        }
                    }
                    GuardMoveType::Turn => State::NotDone,
                },
                None => State::Done,
            };
        }

        let mut res = 0;
        drop(tx);
        while let Some(i) = rx.recv().await {
            res += i;
        }

        println!("Finished receiving");
        Some(res)
    }

    fn day() -> &'static str {
        DAY
    }
}

/*
    PR:
    - 524: Too low
    -1703: GOOD!
*/
#[tokio::main]
async fn main() -> Result<()> {
    simple_logging::log_to_file("test.log", LevelFilter::Info)?;

    start_day(DAY);

    let solution = Day2 {};

    solution.run_tests_part1(TEST, 41).await;

    solution.run_part_1().await?;

    solution.run_tests_part2(TEST, 6).await;

    solution.run_part_2().await?;

    Ok(())
}

#[cfg(test)]
mod test {}
