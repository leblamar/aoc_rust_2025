use std::collections::HashMap;
use std::i64;

use regex::Regex;

use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day10;

#[derive(Debug)]
struct Mask(u32);

impl From<&str> for Mask {
    fn from(value: &str) -> Self {
        let mut res: u32 = 0;
        for (i, c) in value.chars().enumerate() {
            if c == '#' {
                res |= 1 << i;
            }
        }

        Mask(res)
    }
}

#[derive(Debug)]
struct Schema {
    goal: Mask,
    buttons: Vec<Mask>
}

impl From<&str> for Schema {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"\[([.#]+)\]\ ([(),0-9 ]+)\ \{([0-9,]+)\}")
            .expect("Should not have issue with regex");

        let parsed = re.captures(value).expect("Should capture something");
        let goal = Mask::from(&parsed[1]);
        let buttons: Vec<Mask> = parsed[2].split(' ')
            .map(|s| s[1..s.len()-1]
                .split(',')
                .map(|b| b.parse::<usize>().expect("It should parse b correctly"))
                .map(|i| 1 << i)
                .fold(0, |acc, i| acc | i)
            ).map(Mask)
            .collect();
        Schema { goal, buttons }
    }
}

impl Schema {
    fn resolve(&self) -> usize {
        println!("Let's resolve: {:?}", self);

        let mut min = self.buttons.len();
        for i in 0..2_usize.pow(self.buttons.len() as u32) {
            let mut cur_mask: u32 = 0;
            let mut click = 0;
            for j in 0..self.buttons.len() {
                let v = (i >> j) & 1;
                if v == 1 {
                    click += 1;
                    cur_mask ^= self.buttons[j].0;
                }
            }

            if cur_mask == self.goal.0 && click < min {
                min = click;
            } 
        }
        min
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BMask(u128);

impl From<&str> for BMask {
    fn from(value: &str) -> Self {
        let m: u128 = value.split(',')
            .map(|s| s.parse::<u128>().expect("Should be a u8"))
            .enumerate()
            .fold(0, |acc, (i, u)| acc | (u << (i*8)));

        BMask(m)
    }
}

impl BMask {
    fn keep_u8(&self, i: usize) -> u128 {
        let mask = (u8::MAX as u128) << (i * 8);
        self.0 & mask
    }

    fn is_greater_than(&self, o: &Self, len: usize) -> bool {
        (0..len).any(|i| self.keep_u8(i) > o.keep_u8(i))
    }
}

#[derive(Debug)]
struct Schema2 {
    goal: BMask,
    buttons: Vec<BMask>,
    len: usize
}

impl From<&str> for Schema2 {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"\[([.#]+)\]\ ([(),0-9 ]+)\ \{([0-9,]+)\}")
            .expect("Should not have issue with regex");

        let parsed = re.captures(value).expect("Should capture something");
        let buttons: Vec<BMask> = parsed[2].split(' ')
            .map(|s| s[1..s.len()-1]
                .split(',')
                .map(|b| b.parse::<u128>().expect("It should parse b correctly"))
                .fold(0, |acc, u| acc | (1 << (u * 8)))
            )
            .map(BMask)
            .collect();

        let len = parsed[3].split(',').count();

        let goal = BMask::from(&parsed[3]);
        Schema2 { goal, buttons, len }
    }
}

impl Schema2 {
    fn min_path(&self, res_map: &mut HashMap<(u128, u16), u16>, cur_b: &BMask, cur_goal: &BMask, it: u16, min_it: u16) -> u16 {
        let new_it = it + 1;
        if new_it >= min_it {
            return u16::MAX;
        }

        let new_goal = BMask(cur_goal.0 + cur_b.0);
        if new_goal.0 == self.goal.0 {
            return new_it;
        } else if let Some(res) = res_map.get(&(new_goal.0, new_it)) {
            return *res;
        } else if new_goal.is_greater_than(&self.goal, self.len) {
            return u16::MAX;
        }

        let res = self.buttons.iter()
            .fold(min_it, |acc, next_b| {
                let new_min = self.min_path(res_map, next_b, &new_goal, new_it, acc);
                acc.min(new_min)
            });

        res_map.insert((new_goal.0, new_it), res);

        res
    }

    fn slow_resolve(&self) -> u16 {
        let mut res_map: HashMap<(u128, u16), u16> = HashMap::new();
        self.buttons.iter()
            .fold(u16::MAX, |acc, next_b| {
                let new_min = self.min_path(&mut res_map, next_b, &BMask(0),  0, acc);
                acc.min(new_min)
        })
    }

    fn resolve(&self) -> u16 {
        0
    }

}

impl Day10 {
    #[allow(unused)]
    fn slow_part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let schemas: Vec<Schema2> = input.split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(Schema2::from)
            .collect();

        let result: u16 = schemas.iter()
            .map(Schema2::slow_resolve)
            .sum();

        Ok(result as i64)
    }
}

impl Day for Day10 {
    fn get_nb(&self) -> i8 {
        10
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let schemas: Vec<Schema> = input.split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(Schema::from)
            .collect();

        let result: usize = schemas.iter()
            .map(Schema::resolve)
            .sum();

        Ok(result as i64)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let schemas: Vec<Schema2> = input.split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(Schema2::from)
            .collect();

        let result: u16 = schemas.iter()
            .map(Schema2::resolve)
            .sum();

        Ok(result as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day10::Day10;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
        let test_input = 
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
".to_string();
        let result = Day10.part1(test_input).expect("There should be a result");

        assert_eq!(result, 7);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = 
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
".to_string();
        let result = Day10.part2(test_input).expect("There should be a result");

        assert_eq!(result, 33);
    }

    #[test]
    fn it_test_example_simple_part2() {
        let test_input = 
"[.#] (0) (1) (0,1) {2,3}".to_string();
        let result = Day10.part2(test_input).expect("There should be a result");

        assert_eq!(result, 3);
    }

    #[test]
    fn it_test_example_perf_part2() {
        let test_input = 
"[.##...] (1,2) (0,4) (1,3) (1,3,5) {5,38,18,20,5,14}".to_string();
        let result = Day10.part2(test_input).expect("There should be a result");

        assert_eq!(result, 43);
    }

    #[test]
    fn it_test_example_hard_perf_part2() {
        let test_input = 
"[#....#...#] (1,2,3,4,6,7,8) (4,9) (2,3,6,7,9) (0,3,7,8) (0,3,5,8) (0,4,5,6) (4,5,6,8) (1,2,4,6,7,9) {29,3,15,31,45,32,44,31,38,28}".to_string();
        let result = Day10.part2(test_input).expect("There should be a result");

        assert_eq!(result, 43);
    }
}