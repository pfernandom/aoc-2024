use adv_code_2024::*;
use anyhow::*;
use log::LevelFilter;
const DAY: &str = "02";

const TEST: &str = "\
// Test data here
";

struct Day2;

impl Solution<i32> for Day2 {
    fn part1(&self, _input: InputData) -> Option<i32> {
        None
    }

    fn part2(&self, _input: InputData) -> Option<i32> {
        None
    }

    fn day() -> &'static str {
        DAY
    }
}

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

#[cfg(test)]
mod test {

    // tests
}
