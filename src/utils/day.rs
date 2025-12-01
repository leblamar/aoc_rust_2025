use crate::utils::day_error::DayError;

pub trait Day {
    fn get_nb(&self) -> i8;

    fn part1(&self, input: String) -> Result<i64, DayError<'_>>;
    fn part2(&self, input: String) -> Result<i64, DayError<'_>>;
}