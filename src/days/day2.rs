use std::collections::HashMap;

use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day2;

pub struct Range {
    start: i64,
    end: i64
}

impl Range {
    fn create_from(range_str: &str) -> Range {
        let parts: Vec<&str> = range_str.trim().split('-').collect();
        let start: i64 = parts[0].parse().expect("Failed to parse start");
        let end: i64 = parts[1].parse().expect("Failde to parse end");
        Range { start, end }
    }

    fn is_invalid_for_size(chars: &Vec<char>, size: usize) -> bool {
        chars.chunks_exact(size)
            .fold(
                (true, None),
                |acc, chunk| 
                    if let Some(prev) = acc.1 
                        { (acc.0 && (prev == chunk), Some(chunk)) } 
                    else { (true, Some(chunk)) }
            ).0
    }

    fn found_invalid_ids_part1(self) -> Vec<i64> {
        println!("Coucou");
        let mut invalid_ids: Vec<i64> = vec![];

        for i in self.start..=self.end {
            let str_i = i.to_string();
            if str_i.len() % 2 != 0 {
                continue;
            }

            let chars = str_i.chars()
                .collect::<Vec<char>>();
            let is_invalid = Range::is_invalid_for_size(&chars, 2);
            if is_invalid {
                invalid_ids.push(i);
            }

            //let middle = str_i.len() / 2;
            //let left = &str_i[..middle];
            //let right = &str_i[middle..];
            
            //if left == right {
            //    invalid_ids.push(i);
            //}
        }

        invalid_ids
    }

    fn found_invalid_ids_part2(self, dividers_map: &mut HashMap<usize, Vec<usize>>) -> Vec<i64> {
        let mut invalid_ids: Vec<i64> = vec![];

        for i in self.start..=self.end {
            let str_i = i.to_string();
            let len_str = str_i.len();
            let chars = str_i.chars()
                .collect::<Vec<char>>();

            if !dividers_map.contains_key(&len_str) {
                let dividers = (1..len_str).into_iter()
                    .filter(|try_div| len_str % try_div == 0)
                    .collect::<Vec<usize>>();
                dividers_map.insert(len_str, dividers);
            }

            dividers_map.get(&len_str)
                .unwrap()
                .iter()
                .find(|size| Range::is_invalid_for_size(&chars, **size))
                .and_then(|_size| Some(invalid_ids.push(i)));
        }

        invalid_ids
    }
}

impl Day for Day2 {
    fn get_nb(&self) -> i8 {
        2
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let result: i64 = input.split(',')
            .map(Range::create_from)
            .flat_map(Range::found_invalid_ids_part1)
            .sum();

        Ok(result)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let mut dividers_map= HashMap::new();

        let result: i64 = input.split(',')
            .map(Range::create_from)
            .flat_map(|range| range.found_invalid_ids_part2(&mut dividers_map))
            .sum();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day2::Day2;
    use crate::utils::day::Day;

    #[test]
    #[ignore]
    fn it_test_example_part1() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        let result = Day2.part1(test_input).expect("There should be a result");

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        let result = Day2.part2(test_input).expect("There should be a result");

        assert_eq!(result, 4174379265);
    }
}