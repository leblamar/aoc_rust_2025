use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day3;

fn max(sub_str: &str) -> (usize, u64) {
    let mut max_idx: usize = 0;
    let mut max: u64 = 0;
    for (idx, i) in sub_str.chars().map(|val| val.to_digit(10).unwrap() as u64).enumerate() {
        if max < i {
            max = i;
            max_idx = idx;
        }
    }
    (max_idx, max)
}

fn found_max_in_order(nb_to_peak: usize, line: &str) -> u64 {
    let mut result = 0;
    let mut prev_idx_opt = None;
    for i in (0..nb_to_peak).rev() {
        let max_val: u64;
        if let Some(prev_idx) = prev_idx_opt {
            let (idx_max, found_max) = max(&line[prev_idx+1..line.len()-i]);
            prev_idx_opt = Some(idx_max + prev_idx + 1);
            max_val = found_max;
        } else {
            let (idx_max, found_max) = max(&line[..line.len()-i]);
            prev_idx_opt = Some(idx_max);
            max_val = found_max;
        }
        result += max_val * 10_u64.pow(i as u32);
    }
    result
}

impl Day for Day3 {
    fn get_nb(&self) -> i8 {
        3
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let result = input.split("\n")
            .filter(|line| line.trim() != "")
            .map(|line| found_max_in_order(2, line))
            .sum::<u64>();
        Ok(result as i64)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let result = input.split("\n")
            .filter(|line| line.trim() != "")
            .map(|line| found_max_in_order(12, line))
            .sum::<u64>();
        Ok(result as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day3::Day3;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
        let test_input = "987654321111111\n811111111111119\n234234234234278\n818181911112111".to_string();
        let result = Day3.part1(test_input).expect("There should be a result");

        assert_eq!(result, 357);
    }

    #[test]
    fn it_test_part1_hedge() {
        let test_input = "987694321111111".to_string();
        let result = Day3.part1(test_input).expect("There should be a result");

        assert_eq!(result, 99);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = "987654321111111\n811111111111119\n234234234234278\n818181911112111".to_string();
        let result = Day3.part2(test_input).expect("There should be a result");

        assert_eq!(result, 3121910778619);
    }
}