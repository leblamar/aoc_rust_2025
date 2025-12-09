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
        .flat_map(|(i_1, j_1)| j_list2[i_1+1..].iter()
            .enumerate()
            .filter(|(_, j_2)| *j_1 != **j_2)
            .map(|(i_2, j_2)| (i_1, i_2 + i_1 + 1, j_1.dist(j_2)))
            .collect::<Vec<(usize, usize, f64)>>()
        )
        .collect()
}

impl Day for Day8 {
    fn get_nb(&self) -> i8 {
        8
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let nb_iter = 1000;

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
        for (j_1, j_2, _) in closest_pairs {
            let matching_group: Vec<(usize, &RefCell<HashSet<&Junction>>)> = groups.iter()
                .enumerate()
                .filter(|(_, h)| h.borrow().contains(j_1)  || h.borrow().contains(j_2))
                .collect();
            if matching_group.is_empty() {
                let mut new_set: HashSet<&Junction> = HashSet::new();
                new_set.insert(j_1);
                new_set.insert(j_2);
                groups.push(RefCell::new(new_set));
            } else if matching_group.len() == 1 {
                let group = matching_group[0].1;
                group.borrow_mut().insert(j_1);
                group.borrow_mut().insert(j_2);
            } else if matching_group.len() == 2 {
                let g_1 = matching_group[0].1;
                let g_2 = matching_group[1].1;
                g_1.borrow_mut().extend(g_2.borrow().iter());
                groups.remove(matching_group[1].0);

            } else {
                panic!("Len != 2, should not happen");
            }

        }
        let mut result_list: Vec<usize> = groups.iter()
            .map(|r| r.borrow().len()).collect();
        result_list.sort();
        let length = result_list.len();
        let result = result_list[length-3..length].iter()
            .fold(1, |acc, l| acc * l);

        Ok(result as i64)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let junctions: Vec<Junction> = input.split('\n')
            .map(Junction::try_from)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        let mut all_pairs = get_all_pairs(&junctions, &junctions);
        all_pairs.sort_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2));

        let mut groups: Vec<RefCell<HashSet<&Junction>>> = vec![];
        let mut i = 0; // To avoid infinite loop
        let max_length = junctions.len();
        let mut j_set: HashSet<&Junction> = HashSet::new();
        while j_set.len() != max_length && i < 100_000 {
            let pair = all_pairs[i];
            let j_1 = &junctions[pair.0];
            let j_2 = &junctions[pair.1];
            j_set.insert(j_1);
            j_set.insert(j_2);
            
            let matching_group: Vec<(usize, &RefCell<HashSet<&Junction>>)> = groups.iter()
                .enumerate()
                .filter(|(_, h)| h.borrow().contains(j_1)  || h.borrow().contains(j_2))
                .collect();
            if matching_group.is_empty() {
                let mut new_set: HashSet<&Junction> = HashSet::new();
                new_set.insert(j_1);
                new_set.insert(j_2);
                groups.push(RefCell::new(new_set));
            } else if matching_group.len() == 1 {
                let group = matching_group[0].1;
                group.borrow_mut().insert(j_1);
                group.borrow_mut().insert(j_2);
            } else if matching_group.len() == 2 {
                let g_1 = matching_group[0].1;
                let g_2 = matching_group[1].1;
                g_1.borrow_mut().extend(g_2.borrow().iter());
                groups.remove(matching_group[1].0);
            } else {
                panic!("Len != 2, should not happen");
            }

            i += 1;
        }

        let last_pair = all_pairs[i-1];
        let j_1 = &junctions[last_pair.0];
        let j_2 = &junctions[last_pair.1];
        let result = j_1.x * j_2.x;

        Ok(result as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day8::Day8;
    use crate::utils::day::Day;

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
    fn it_test_example_part2() {
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
        let result = Day8.part2(test_input).expect("There should be a result");

        assert_eq!(result, 25272);
    }
}