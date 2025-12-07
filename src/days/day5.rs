use std::ops::{Bound, RangeBounds};

use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day5;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64
}

impl RangeBounds<i64> for Range {
    fn contains<U>(&self, item: &U) -> bool
        where
            i64: PartialOrd<U>,
            U: ?Sized + PartialOrd<i64>, {
        self.start <= *item && *item <= self.end
    }

    fn start_bound(&self) -> std::ops::Bound<&i64> {
        Bound::Included(&self.start)
    }
    
    fn end_bound(&self) -> Bound<&i64> {
        Bound::Included(&self.end)
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let values = value.split('-').collect::<Vec<&str>>();
        if values.len() != 2 {
            panic!("This should not happend");
        }

        let start = values[0].parse().expect("This should be a number");
        let end = values[1].parse().expect("This should be a number");
        Range { start, end }
    }
}

impl Range {
    fn overlap_with(&self, other: &Range) -> bool {
        self.contains(&other.start) || self.contains(&other.end)
            || other.contains(&self.start) || other.contains(&self.end)
    }

    fn try_merge(&self, other: &Range) -> Option<Range> {
        if !self.overlap_with(&other) {
            return None;
        }

        let m_start = self.start.min(other.start);
        let m_end = self.end.max(other.end);

        Some(Range { start: m_start, end: m_end })
    }

    fn size(&self) -> i64 {
        self.end - self.start + 1
    }
}

impl Day for Day5 {
    fn get_nb(&self) -> i8 {
        5
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
        let ranges = ranges_str.split('\n').map(Range::from).collect::<Vec<Range>>();

        let result = ids_str.split('\n')
            .filter(|id_str| id_str.trim() != "")
            .map(|id_str| id_str.parse::<i64>().unwrap())
            .filter(|id| ranges.iter().any(|range| range.contains(&id)))
            .count();
        
        Ok(result as i64)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let (ranges_str, _) = input.split_once("\n\n").unwrap();
        let ranges = ranges_str.split('\n').map(Range::from).collect::<Vec<Range>>();

        let mut merged_ranges: Vec<Range> = vec![];
        for range in ranges {
            let mut to_managed = vec![range];

            while let Some(range_to_treat) = to_managed.pop() {
                let find_idx_opt = merged_ranges.iter()
                    .position(|m_range| range_to_treat.overlap_with(m_range));
                match find_idx_opt {
                    Some(find_idx) => {
                        // If found a merge, then must reset the merged_ranges and refill the range_to_treat
                        let find_merge = &merged_ranges.remove(find_idx);
                        let merged_range = find_merge.try_merge(&range_to_treat).expect("This should not happen");
                        to_managed.append(&mut merged_ranges);
                        to_managed.push(merged_range);
                        merged_ranges.clear();
                    },
                    None => merged_ranges.push(range_to_treat)
                }
            }
        }

        let result = merged_ranges.iter().map(Range::size).sum::<i64>();
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day5::Day5;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
        let test_input = 
"3-5
10-14
16-20
12-18

1
5
8
11
17
32".to_string();
        let result = Day5.part1(test_input).expect("There should be a result");

        assert_eq!(result, 3);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = 
"3-5
10-14
16-20
12-18

1
5
8
11
17
32".to_string();
        let result = Day5.part2(test_input).expect("There should be a result");

        assert_eq!(result, 14);
    }
}