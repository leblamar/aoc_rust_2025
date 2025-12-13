use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day12;

#[allow(unused)]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64
}

#[allow(unused)]
#[derive(Debug)]
struct Present {
    i: usize,
    shape: Vec<Point>
}

impl From<&str> for Present {
    fn from(value: &str) -> Self {
        let (i_str, shape_str) = value.split_once(":\n").expect("Should be able to split on ':\\n'");
        let i = i_str.parse().expect("Should be able to parse idx");
        let shape = shape_str.split('\n')
            .enumerate()
            .flat_map(|(i, r_str)| r_str.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(j, _)| Point { x: j as i64, y: i as i64 })
                .collect::<Vec<Point>>()
            )
            .collect();
        Present { i, shape }
    }
}

#[derive(Debug)]
struct Tree {
    h: usize,
    w: usize,
    ids: Vec<usize>
}

impl From<&str> for Tree {
    fn from(value: &str) -> Self {
        let (h_w_str, ids_str) = value.split_once(": ").expect("It should have ': '");

        let (w_str, h_str) = h_w_str.split_once('x').expect("It should have a 'x'");
        let h = h_str.parse().expect("h should be a usize");
        let w = w_str.parse().expect("w swould be a usize");

        let ids = ids_str.split(' ')
            .map(|i_str| i_str.parse().expect("ids should be usize"))
            .collect();

        Tree { h, w, ids }
    }
}


impl Tree {
    fn is_valid(&self, presents: &Vec<Present>) -> bool {
        println!("Curr tree:\n{:?}", self);
        let filtered: Vec<(usize, &Present)> = self.ids.iter()
            .enumerate()
            .filter(|(_, id)| **id != 0)
            .map(|(i, id)| (*id, &presents[i]))
            .collect();

        let min_s: usize = filtered.iter().map(|(i, p)| i * p.shape.len()).sum();
        if self.h * self.w < min_s {
            return false;
        }

        
        
        println!("Filtered:\n{:?}", filtered);
        false
    }
}

impl Day for Day12 {
    fn get_nb(&self) -> i8 {
        12
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        println!("Input:\n{input}");
        let first_split: Vec<&str> = input.split("\n\n").collect();
        let presents: Vec<Present> = first_split[0..first_split.len()-1]
            .iter()
            .map(|s| Present::from(*s))
            .collect();

        let trees: Vec<Tree> = first_split[first_split.len()-1].split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(Tree::from)
            .collect();

        println!("Presents:\n{:?}", presents);
        println!("Trees:\n{:?}", trees);

        let result = trees[..1].iter()
            .filter(|t| t.is_valid(&presents))
            .count();

        Ok(result as i64)
    }

    fn part2(&self, _input: String) -> Result<i64, DayError<'_>> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day12::Day12;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
        let test_input = 
"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2".to_string();
        let result = Day12.part1(test_input).expect("There should be a result");

        assert_eq!(result, 2);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = 
"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2".to_string();
        let result = Day12.part2(test_input).expect("There should be a result");

        assert_eq!(result, 40);
    }
}