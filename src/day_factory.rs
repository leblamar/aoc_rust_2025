use crate::days;
use crate::utils;

pub fn create_day(day: i8) -> Option<&'static dyn utils::day::Day> {
    if day == 1 {
        return Some(&days::day1::Day1);
    } else if day == 2 {
        return Some(&days::day2::Day2);
    } else if day == 3 {
        return Some(&days::day3::Day3);
    } else if day == 4 {
        return Some(&days::day4::Day4);
    } else if day == 5 {
        return Some(&days::day5::Day5);
    }

    None
}