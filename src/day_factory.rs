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
    } else if day == 6 {
        return Some(&days::day6::Day6);
    } else if day == 7 {
        return Some(&days::day7::Day7);
    } else if day == 8 {
        return Some(&days::day8::Day8);
    } else if day == 9 {
        return Some(&days::day9::Day9);
    } else if day == 10 {
        return Some(&days::day10::Day10);
    } else if day == 11 {
        return Some(&days::day11::Day11);
    } else if day == 12 {
        return Some(&days::day12::Day12);
    }

    None
}