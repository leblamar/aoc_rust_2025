use std::cell::RefCell;
use std::collections::HashSet;

use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Junction {
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug)]
struct Day8Error {}

impl TryFrom<&str> for Junction {
    type Error = Day8Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s: Vec<i64> = value.split(',')
            .filter(|c| !c.trim().is_empty())
            .map(|c| c.parse::<i64>())
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
        if s.len() != 3 {
            return Err(Day8Error {  });
        }

        Ok(Junction { x: s[0], y: s[1], z: s[2] })
    }
}

impl Junction {
    fn dist(&self, o: &Junction) -> f64 {
        ((((self.x - o.x).pow(2) + (self.y - o.y).pow(2) + (self.z - o.z).pow(2))) as f64).sqrt()
    }
}

fn get_all_pairs(j_list1: &Vec<Junction>, j_list2: &Vec<Junction>) -> Vec<(usize, usize, f64)> {
    j_list1.iter()
        .enumerate()
        .flat_map(|(i_1, j_1)| j_list2.iter()
            .enumerate()
            .filter(|(_, j_2)| *j_1 != **j_2)
            .map(|(i_2, j_2)| (i_1, i_2, j_1.dist(j_2)))
            .collect::<Vec<(usize, usize, f64)>>()
        )
        .collect()
}

impl Day for Day8 {
    fn get_nb(&self) -> i8 {
        8
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        println!("Input:\n{input}");
        let nb_iter = 11;

        let junctions: Vec<Junction> = input.split('\n')
            .map(Junction::try_from)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        let mut all_pairs = get_all_pairs(&junctions, &junctions);
        all_pairs.sort_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2));
        let closest_pairs: Vec<(&Junction, &Junction, f64)> = all_pairs[..nb_iter].iter()
            .map(|(i1, i2, d)| (&junctions[*i1], &junctions[*i2], *d))
            .collect();

        let mut groups: Vec<RefCell<HashSet<&Junction>>> = vec![];
        println!("Closest pairs: {:?}", closest_pairs);
        let mut i = 0;
        let max_length = junctions.len();
        let mut j_set: HashSet<&Junction> = HashSet::new();
        while j_set.len() != max_length && i < 10 {
            i += 1;
            //println!("Iteration: {i}");
            let pair = all_pairs[i];
            let j_1 = &junctions[pair.0];
            let j_2 = &junctions[pair.1];
            j_set.insert(j_1);
            j_set.insert(j_2);
            //println!("j_set: {:?}", j_set.len());
            //println!("max_length: {max_length}");
            
            let group_opt = groups.iter()
                .find(|h| h.borrow().contains(j_1) || h.borrow().contains(j_2));
            if let Some(group) = group_opt {
                group.borrow_mut().insert(j_1);
                group.borrow_mut().insert(j_2);
            } else {
                let mut new_set: HashSet<&Junction> = HashSet::new();
                new_set.insert(j_1);
                new_set.insert(j_2);
                groups.push(RefCell::new(new_set));
            }

        }
        println!("Groups:\n{:?}", groups);
        let mut result_list: Vec<usize> = groups.iter()
            .map(|r| {
                let val = r.borrow().len();
                println!("Val: {val}");
                val
            }).collect();
        result_list.sort();
        let length = result_list.len();
        let result = result_list[length-3..length].iter()
            .fold(1, |acc, l| {
                println!("Length: {l}");
                acc * l
            });

        Ok(result as i64)
    }

    fn part2(&self, _input: String) -> Result<i64, DayError<'_>> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day8::{Day8, Junction};
    use crate::utils::day::Day;

    #[test]
    fn it_test_dist() {
        let j_1 = Junction { x: 162, y: 817, z: 812 };
        let j_2 = Junction { x: 425, y: 690, z: 689 };
        let j_3 = Junction { x: 57, y: 618, z: 57 };
        println!("Dist j_1, j_2: {}", j_1.dist(&j_2));
        println!("Dist j_1, j_3: {}", j_1.dist(&j_3));
        println!("Dist j_2, j_3: {}", j_2.dist(&j_3));
        assert!(false);
    }

    #[test]
    fn it_test_example_part1() {
        let test_input = 
"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689".to_string();
        let result = Day8.part1(test_input).expect("There should be a result");

        assert_eq!(result, 40);
    }

    #[test]
    fn it_test_example_part2_simple() {
        let test_input = 
".......S.......
...............
.......^.......
...............".to_string();
        let result = Day8.part2(test_input).expect("There should be a result");

        assert_eq!(result, 2);
    }

    #[test]
    fn it_test_example_part2_medium() {
        let test_input = 
".......S.......
...............
.......^.......
...............
......^.^......
...............".to_string();
        let result = Day8.part2(test_input).expect("There should be a result");

        assert_eq!(result, 4);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = 
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............".to_string();
        let result = Day8.part2(test_input).expect("There should be a result");

        assert_eq!(result, 40);
    }
}