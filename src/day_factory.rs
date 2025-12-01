use crate::days;
use crate::utils;

pub fn create_day(day: i8) -> Option<&'static dyn utils::day::Day> {
    if day == 1 {
        return Some(&days::day1::Day1);
    }

    None
}