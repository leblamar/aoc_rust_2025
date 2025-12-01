use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day1;

impl Day for Day1 {
    fn get_nb(&self) -> i8 {
        1
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        println!("Day 1 part 1");
        println!("Input: {input}");
        Ok(1)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        println!("Day 1 part 2");
        println!("Input: {input}");
        Ok(2)
    }
}