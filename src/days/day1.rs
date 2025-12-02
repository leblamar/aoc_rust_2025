use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day1;

impl Day for Day1 {
    fn get_nb(&self) -> i8 {
        1
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        println!("Day 1 part 1");

        let test_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();

        let mut dial = 50;

        let mut count_0 = 0;
        for rotation in input.split("\n") {
            if rotation == "" {
                continue;
            }
            let value: i64 = rotation[1..].parse().unwrap();
            if rotation.starts_with('L') {
                dial = (dial - value) % 100;
            } else if rotation.starts_with('R') {
                dial = (dial + value) % 100;
            } else {
                panic!("Your rotation should start with L or R and not: {rotation}")
            }

            if dial == 0 {
                count_0 += 1;
            }
        }

        Ok(count_0)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        println!("Day 1 part 2");

        //let test_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string(); //6
        //let test_input = "L50\nL100".to_string(); //2
        //let test_input = "L50\nL1".to_string(); //1
        //let test_input = "L50\nR50".to_string(); //1
        //let test_input = "L50\nR101".to_string(); //2
        //let test_input = "R50\nR100".to_string(); //2
        //let test_input = "R50\nR1".to_string(); //1
        //let test_input = "L100\nR151".to_string(); //3
        //let test_input = "L7\nL743".to_string(); //8
        //let test_input = "L7\nL143".to_string(); //2
        //let test_input = "L7\nL142".to_string(); //1
        //let test_input = "R7\nR143".to_string(); //2
        //let test_input = "L57\nL93".to_string(); //2
        //let test_input = "L57\nL193".to_string(); //3
        //let test_input = "L57\nL192".to_string(); //2
        //let test_input = "L57\nR107".to_string(); //3
        //let test_input = "L57\nR108".to_string(); //3
        //let test_input = "L57\nR106".to_string(); //2
        //let test_input = "L57\nR207".to_string(); //4

        let mut dial = 50;

        let mut count_0: i64 = 0;
        for rotation in input.split("\n") {
            if rotation == "" {
                continue;
            }
            let value: i64 = rotation[1..].parse().unwrap();
            if rotation.starts_with('L') {
                if 0 < dial && dial - value < 0 { 
                    let should_add_one = (dial - value) % 100 != 0;
                    count_0 += (dial - value).abs() / 100 + if should_add_one { 1 } else { 0 };
                } else if dial <= 0 && dial - value < -100 {
                    let should_sub_one = (dial - value) % 100 == 0;
                    count_0 += (dial - value).abs() / 100 - if should_sub_one { 1 } else { 0 };
                }
                dial = (dial - value) % 100;
            } else if rotation.starts_with('R') {
                if dial < 0 && 0 < dial + value { 
                    let should_add_one = (dial + value) % 100 != 0;
                    count_0 += (dial + value) / 100 + if should_add_one { 1 } else { 0 };
                } else if 0 <= dial && 100 < dial + value {
                    let should_sub_one = (dial + value) % 100 == 0;
                    count_0 += (dial + value) / 100 - if should_sub_one { 1 } else { 0 };
                }
                dial = (dial + value) % 100;
            } else {
                panic!("Your rotation should start with L or R and not: {rotation}")
            }

            if dial == 0 {
                count_0 += 1;
            }
        }

        Ok(count_0)
    }
}