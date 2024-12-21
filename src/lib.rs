use anyhow::*;
use code_timing_macros::time_snippet;
use ndarray::Array2;
use std::{
    error::Error,
    fmt::{self, Debug, Display},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
pub mod map;

#[derive(Clone, Debug)]
pub struct FileNotFoundError {
    path: String,
}

impl FileNotFoundError {
    fn new(path: String) -> FileNotFoundError {
        return FileNotFoundError { path };
    }
}

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "input file not found: {}", self.path)
    }
}

impl Error for FileNotFoundError {}

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

pub fn iter_from_str(input: &'static str) -> InputData {
    InputData {
        data: input
            .lines()
            .map(|l| String::from(l.trim()))
            .filter(|l| !l.is_empty())
            .collect(),
    }
}

pub struct InputData {
    data: Vec<String>,
}

impl InputData {
    pub fn from_str(input: &'static str) -> InputData {
        InputData {
            data: input
                .lines()
                .map(|l| String::from(l.trim()))
                .filter(|l| !l.is_empty())
                .collect(),
        }
    }
    pub fn from_path(path: &Path) -> Result<InputData> {
        let f = File::open(path)?;

        let reader = BufReader::new(f);

        Ok(InputData {
            data: reader.lines().filter_map(|l| l.ok()).collect(),
        })
    }

    pub fn lines(&self) -> impl Iterator<Item = &String> {
        self.data.iter()
    }
}

pub fn read_input(day: &str) -> Result<InputData> {
    let str_path = ["input/", day, ".txt"].concat();
    let path = Path::new(&str_path);
    if !path.exists() {
        return Err(FileNotFoundError::new(str_path).into());
    }

    InputData::from_path(path)
}

pub trait Solution<R>
where
    R: Eq + Debug + Display,
{
    fn day() -> &'static str;
    fn part1(&self, input: InputData) -> Option<R>;
    fn part2(&self, input: InputData) -> Option<R>;
    fn run(&self) -> Result<()> {
        self.run_part_1()?;
        self.run_part_2()?;

        Ok(())
    }

    fn run_part_1(&self) -> Result<()> {
        match read_input(Self::day()) {
            std::result::Result::Ok(input) => {
                println!("\n=== Part 1 ===");
                match time_snippet!(self.part1(input)) {
                    Some(solution) => {
                        println!("Solution P1: {}", solution);
                    }
                    None => println!("Part 1 not implemented yet"),
                }
            }
            Err(e) => {
                println!("Could not read input for part 1: {:?}", e);
            }
        }

        Ok(())
    }

    fn run_and_assert_part_1(&self, expected: R) -> Result<()> {
        match read_input(Self::day()) {
            std::result::Result::Ok(input) => {
                println!("\n=== Part 1 ===");
                match time_snippet!(self.part1(input)) {
                    Some(solution) => {
                        println!("Solution P1: {}", solution);
                        assert_eq!(expected, solution);
                    }
                    None => println!("Part 1 not implemented yet"),
                }
            }
            Err(e) => {
                println!("Could not read input for part 1: {:?}", e);
            }
        }

        Ok(())
    }

    fn run_part_2(&self) -> Result<()> {
        if let std::result::Result::Ok(input) = read_input(Self::day()) {
            println!("\n=== Part 2 ===");
            match time_snippet!(self.part2(input)) {
                Some(solution) => {
                    println!("Solution P2: {}", solution);
                }
                None => println!("Part 2 not implemented yet"),
            }

            return Ok(());
        }

        println!("Could not read input for part 2");
        Ok(())
    }

    fn run_tests_part1(&self, input: &'static str, expected: R) {
        println!("\n=== Part 1 (test) ===");
        let it = iter_from_str(input);
        match time_snippet!(self.part1(it)) {
            Some(solution) => {
                assert_eq!(
                    expected, solution,
                    "expected: {}, actual: {}",
                    expected, solution
                );
                println!("Test successful");
            }
            None => println!("Part 1 not implemented yet"),
        }
    }

    fn run_tests_part2(&self, input: &'static str, expected: R) {
        println!("\n=== Part 2 (test) ===");
        let it = iter_from_str(input);
        match time_snippet!(self.part2(it)) {
            Some(solution) => {
                assert_eq!(
                    expected, solution,
                    "expected: {}, actual: {}",
                    expected, solution
                );
                println!("Test successful");
            }
            None => println!("Part 1 not implemented yet"),
        }
    }

    fn run_tests<'a>(&self, input: &'static str, expected1: R, expected2: R) -> Result<()> {
        self.run_tests_part1(input, expected1);
        self.run_tests_part2(input, expected2);

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait AsyncSolution<R>
where
    R: Eq + Debug + Display + Send,
{
    fn day() -> &'static str;
    async fn part1(&self, input: InputData) -> Option<R>;
    async fn part2(&self, input: InputData) -> Option<R>;
    async fn run(&self) -> Result<()> {
        self.run_part_1().await?;
        self.run_part_2().await?;

        Ok(())
    }

    async fn run_part_1(&self) -> Result<()> {
        match read_input(Self::day()) {
            std::result::Result::Ok(input) => {
                println!("\n=== Part 1 ===");
                match time_snippet!(self.part1(input).await) {
                    Some(solution) => {
                        println!("Solution P1: {}", solution);
                    }
                    None => println!("Part 1 not implemented yet"),
                }
            }
            Err(e) => {
                println!("Could not read input for part 1: {:?}", e);
            }
        }

        Ok(())
    }

    async fn run_part_2(&self) -> Result<()> {
        if let std::result::Result::Ok(input) = read_input(Self::day()) {
            println!("\n=== Part 2 ===");
            match time_snippet!(self.part2(input).await) {
                Some(solution) => {
                    println!("Solution P2: {}", solution);
                }
                None => println!("Part 2 not implemented yet"),
            }

            return Ok(());
        }

        println!("Could not read input for part 2");
        Ok(())
    }

    async fn run_tests_part1(&self, input: &'static str, expected: R)
    where
        R: 'async_trait,
    {
        println!("\n=== Part 1 (test) ===");
        let it = iter_from_str(input);
        match time_snippet!(self.part1(it).await) {
            Some(solution) => {
                assert_eq!(
                    expected, solution,
                    "expected: {}, actual: {}",
                    expected, solution
                );
                println!("Test successful");
            }
            None => println!("Part 1 not implemented yet"),
        }
    }

    async fn run_tests_part2(&self, input: &'static str, expected: R)
    where
        R: 'async_trait,
    {
        println!("\n=== Part 2 (test) ===");
        let it = iter_from_str(input);
        match time_snippet!(self.part2(it).await) {
            Some(solution) => {
                assert_eq!(
                    expected, solution,
                    "expected: {}, actual: {}",
                    expected, solution
                );
                println!("Test successful");
            }
            None => println!("Part 1 not implemented yet"),
        }
    }

    async fn run_tests(&self, input: &'static str, expected1: R, expected2: R) -> Result<()>
    where
        R: 'async_trait,
    {
        self.run_tests_part1(input, expected1).await;
        self.run_tests_part2(input, expected2).await;

        Ok(())
    }
}

// Additional common functions

pub fn parse_u32_matrix<T: AsRef<str>>(input: Vec<T>) -> Array2<u32> {
    // Get the number of rows and columns
    let rows = input.len();
    let cols = input.first().map_or(0, |row| row.as_ref().len());

    // Flatten the input into a single vector of characters
    let flattened: Vec<u32> = input
        .iter()
        .flat_map(|row| {
            row.as_ref()
                .chars()
                .map(|e| e.to_digit(10).expect("to be a digit"))
        })
        .collect();

    // Ensure the dimensions match
    assert_eq!(
        flattened.len(),
        rows * cols,
        "Input is not a rectangular matrix"
    );

    // Create the 2D array from the flattened data
    Array2::from_shape_vec((rows, cols), flattened).expect("Failed to create Array2")
}

pub fn parse_char_matrix<T: AsRef<str>>(input: Vec<T>) -> Array2<char> {
    // Get the number of rows and columns
    let rows = input.len();
    let cols = input.first().map_or(0, |row| row.as_ref().len());

    // Flatten the input into a single vector of characters
    let flattened: Vec<char> = input.iter().flat_map(|row| row.as_ref().chars()).collect();

    // Ensure the dimensions match
    assert_eq!(
        flattened.len(),
        rows * cols,
        "Input is not a rectangular matrix"
    );

    // Create the 2D array from the flattened data
    Array2::from_shape_vec((rows, cols), flattened).expect("Failed to create Array2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
