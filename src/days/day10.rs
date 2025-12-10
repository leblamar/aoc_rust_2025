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
        let re = Regex::new(r"\[([.#]+)\]\ ([(),0-9 ]+)\ (\{[0-9,]+\})")
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

    fn part2(&self, _input: String) -> Result<i64, DayError<'_>> {
        Ok(0)
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

        assert_eq!(result, 40);
    }
}