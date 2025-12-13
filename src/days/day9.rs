use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day9;

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
struct Tile {
    x: i64,
    y: i64
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        let (x_str, y_str) = value.split_once(',').expect("' not found");
        Tile { x: x_str.parse().expect("x unwrap"), y: y_str.parse().expect("y unwrap") }
    }
}

impl Tile {
    fn area(&self, o: &Tile) -> i64 {
        ((self.x - o.x).abs() + 1) * ((self.y - o.y).abs() + 1)
    }

    fn is_aligned(&self, o: &Tile) -> bool {
        self.x == o.x || self.y == o.y
    }

    fn get_edge(&self, o: &Tile) -> Vec<Tile> {
        if self.x == o.x {
            let min_y = self.y.min(o.y);
            let max_y = self.y.max(o.y);
            (min_y..=max_y).map(|y_i| Tile { x: self.x, y: y_i }).collect()
        } else if self.y == o.y {
            let min_x = self.x.min(o.x);
            let max_x = self.x.max(o.x);
            (min_x..=max_x).map(|x_i| Tile { x: x_i, y: self.y }).collect()
        } else {
            vec![]
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Square {
    f: Tile,
    s: Tile,
}

impl Square {
    fn create_square(
        x_map: &HashMap<i64, (i64, i64)>, 
        y_map: &HashMap<i64, (i64, i64)>, 
        t1: &Tile, 
        t2: &Tile
    ) -> Option<Square> {
        let (a1, a2) = Square::angles(t1, t2);

        let (y1_min, y1_max) = x_map[&a1.x];
        if a1.y < y1_min || y1_max < a1.y {
            return None;
        }

        let (x1_min, x1_max) = y_map[&a1.y];
        if a1.x < x1_min || x1_max < a1.x {
            return None;
        }

        let (y2_min, y2_max) = x_map[&a2.x];
        if a2.y < y2_min || y2_max < a2.y {
            return None;
        }

        let (x2_min, x2_max) = y_map[&a2.y];
        if a2.x < x2_min || x2_max < a2.x {
            return None;
        }

        Some(Square { f: t1.clone(), s: t2.clone() })
    }

    fn angles(t_1: &Tile, t_2: &Tile) -> (Tile, Tile) {
        (Tile { x: t_1.x, y: t_2.y }, Tile { x: t_2.x, y: t_1.y} )
    }

    fn size(&self) -> i64 {
        self.f.area(&self.s)
    }

    fn is_valid(&self, x_map: &HashMap<i64, (i64, i64)>, y_map: &HashMap<i64, (i64, i64)>) -> bool {
        let t1 = &self.f;
        let t2 = &self.s;
        let (min_x, max_x) = if t1.x < t2.x { (t1.x, t2.x) } else { (t2.x, t1.x) };
        let (min_y, max_y) = if t1.y < t2.y { (t1.y, t2.y) } else { (t2.y, t1.y) };

        for xi in min_x..=max_x {
            let (yi_min, yi_max) = x_map[&xi];
            if min_y < yi_min || yi_max < max_y {
                return false;
            }
        }

        for yi in min_y..=max_y {
            let (xi_min, xi_max) = y_map[&yi];
            if min_x < xi_min || xi_max < max_x {
                return false;
            }
        }

        true
    }
}

impl Day for Day9 {
    fn get_nb(&self) -> i8 {
        9
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let tiles: Vec<Tile> = input.split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(Tile::from)
            .collect();

        let result = tiles.iter()
            .flat_map(|t_1| tiles.iter().map(|t_2| t_1.area(t_2)))
            .max()
            .expect("Max not found");

        Ok(result)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let tiles: Vec<Tile> = input.split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(Tile::from)
            .collect();

        let perimeter: HashSet<Tile> = tiles.iter()
            .circular_tuple_windows()
            .flat_map(|(t_1, t_2)| t_1.get_edge(t_2))
            .collect();

        let (x_map, y_map) = perimeter.iter()
            .fold((HashMap::new(), HashMap::new()), |(mut x_acc, mut y_acc), t| {
                if let Some((y_min, y_max)) = x_acc.get(&t.x) {
                    if t.y < *y_min {
                        x_acc.insert(t.x, (t.y, *y_max));
                    } else if *y_max < t.y {
                        x_acc.insert(t.x, (*y_min, t.y));
                    }
                } else {
                    x_acc.insert(t.x, (t.y, t.y));
                }

                if let Some((x_min, x_max)) = y_acc.get(&t.y) {
                    if t.x < *x_min {
                        y_acc.insert(t.y, (t.x, *x_max));
                    } else if *x_max < t.x {
                        y_acc.insert(t.y, (*x_min, t.x));
                    }
                } else {
                    y_acc.insert(t.y, (t.x, t.x));
                }

                return (x_acc, y_acc);
            });
        
        let res_square = tiles.iter()
            .enumerate()
            .map(|(i_1, t_1)|
                tiles[i_1+1..].iter()
                    .filter(|t_2| !t_1.is_aligned(t_2))
                    .map(|t_2| Square::create_square(&x_map, &y_map, t_1, t_2))
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .filter(|s| s.is_valid(&x_map, &y_map))
                    .max_by_key(Square::size)
            ).filter(Option::is_some)
            .map(Option::unwrap)
            .max_by_key(Square::size)
            .expect("Should have at least one max");

        Ok(res_square.size())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day9::Day9;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
        let test_input = 
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3".to_string();
        let result = Day9.part1(test_input).expect("There should be a result");

        assert_eq!(result, 50);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = 
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3".to_string();
        let result = Day9.part2(test_input).expect("There should be a result");

        assert_eq!(result, 24);
    }

    #[test]
    fn it_test_example_simple_part2() {
        let test_input = 
"2,1
12,1
12,3
8,3
8,5
2,5".to_string();
        let result = Day9.part2(test_input).expect("There should be a result");

        assert_eq!(result, 35);
    }

    #[test]
    fn it_test_example_medium_part2() {
        let test_input = 
"2,1
12,1
12,3
10,3
10,5
12,5
12,7
2,7".to_string();
        let result = Day9.part2(test_input).expect("There should be a result");

        assert_eq!(result, 45);
    }
}