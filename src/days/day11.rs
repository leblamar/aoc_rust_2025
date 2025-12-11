use std::collections::HashMap;

use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day11;

#[derive(Debug, Clone)]
struct Device {
    id: String,
    outputs: Vec<String>
}

impl From<&str> for Device {
    fn from(value: &str) -> Self {
        let (id, outputs_str) = value.split_once(": ").expect("Should have a name and outputs");
        let outputs: Vec<String> = outputs_str.split(' ').map(str::to_string).collect();

        Device { id: id.to_string(), outputs }
    }
}

fn solve(d_map: &HashMap<String, Device>, d_str: &String, r_map: &mut HashMap<String, i64>) -> i64 {
    if d_str == "out" {
        return 1;
    } else if let Some(res) = r_map.get(d_str) {
        return *res;
    }


    let res = d_map[d_str].outputs
        .iter()
        .map(|next_d| solve(d_map, next_d, r_map))
        .sum();

    r_map.insert(d_str.clone(), res);
    res
}

fn solve_v2(
    d_map: &HashMap<String, Device>, 
    d_str: &String, 
    r_map: &mut HashMap<(bool, bool, String), i64>, 
    f_fft: bool, 
    f_dac: bool
) -> i64 {
    if d_str == "out" {
        return if f_fft && f_dac { 1 } else { 0 };
    } else if let Some(res) = r_map.get(&(f_fft, f_dac, d_str.clone())) {
        return *res;
    }

    let next_fft = f_fft || d_str == "fft";
    let next_dac = f_dac || d_str == "dac";

    let res = d_map[d_str].outputs
        .iter()
        .map(|next_d| solve_v2(d_map, next_d, r_map, next_fft, next_dac))
        .sum();

    r_map.insert((next_fft, next_dac, d_str.clone()), res);
    res
}

impl Day for Day11 {
    fn get_nb(&self) -> i8 {
        11
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let d_map: HashMap<String, Device> = input.split('\n')
            .filter(|d| !d.trim().is_empty())
            .map(Device::from)
            .map(|d| (d.id.clone(), d))
            .collect();

        let s_d = "you".to_string();
        let mut r_map: HashMap<String, i64> = HashMap::new();

        let result = solve(&d_map, &s_d, &mut r_map);

        Ok(result)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let d_map: HashMap<String, Device> = input.split('\n')
            .filter(|d| !d.trim().is_empty())
            .map(Device::from)
            .map(|d| (d.id.clone(), d))
            .collect();

        let s_d = "svr".to_string();
        let mut r_map: HashMap<(bool, bool, String), i64> = HashMap::new();

        let result = solve_v2(&d_map, &s_d, &mut r_map, false, false);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day11::Day11;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
        let test_input = 
"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out".to_string();
        let result = Day11.part1(test_input).expect("There should be a result");

        assert_eq!(result, 5);
    }

    #[test]
    fn it_test_example_harder_part1() {
        let test_input = 
"you: eee ccc
ccc: eee fff
eee: ee1 ee2
ee1: out
ee2: out
fff: out".to_string();
        let result = Day11.part1(test_input).expect("There should be a result");

        assert_eq!(result, 5);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = 
"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out".to_string();
        let result = Day11.part2(test_input).expect("There should be a result");

        assert_eq!(result, 2);
    }
}