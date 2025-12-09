use std::collections::HashSet;
use std::fmt::Display;

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
        (self.x - o.x + 1).abs() * (self.y - o.y + 1).abs()
    }
}

#[derive(Debug)]
enum T {
    Red,
    Green,
    Dot
}

impl Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            T::Red => write!(f, "O"),
            T::Green => write!(f, "X"),
            T::Dot => write!(f, ".")
        }
    }
}

#[derive(Debug)]
struct Matrix {
    tiles: HashSet<Tile>,
    m: Vec<Vec<T>>,
    h: usize,
    w: usize
}

impl From<&Vec<Tile>> for Matrix {
    fn from(value: &Vec<Tile>) -> Self {
        let h = value.iter().map(|t| t.y).max().expect("h not found") as usize + 2 as usize;
        let w = value.iter().map(|t| t.x).max().expect("h not found") as usize + 2 as usize;
        let m: Vec<Vec<T>> = (0..h).map(|_i| 
                (0..w).map(|_j| T::Dot).collect::<Vec<T>>()
            ).collect();
        let mut tiles = HashSet::new();
        value.iter().for_each(|t| { tiles.insert(t.clone()); });
        Matrix { tiles, m, h, w }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;
        for i in 0..self.w {
            if i < 10 {
                write!(f, "  {}", i)?;
            } else {
                write!(f, " {}", i)?;
            }
        }
        writeln!(f, "")?;
        for i in 0..self.h {
            write!(f, "{}", i)?;
            for j in 0..self.w {
                let c = &self.m[i][j];
                if self.tiles.contains(&Tile { x: j as i64, y: i as i64 }) {
                    write!(f, "  #")?;
                } else {
                    write!(f, "  {}", c)?;
                }
            }
            writeln!(f, )?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Square {
    first: Tile,
    second: Tile,
    side: Tile
}

impl Square {
    fn create_square(t_1: &Tile, t_2: &Tile, t_3: &Tile) -> Option<Square> {
        if t_1.x == t_2.x {
            if t_3.y == t_1.y {
                return if t_2 < t_3 { 
                    Some(Square { first: t_2.clone(), second: t_3.clone(), side: t_1.clone() }) 
                } else {
                    Some(Square { first: t_3.clone(), second: t_2.clone(), side: t_1.clone() }) 
                }
            } else if t_3.y == t_2.y {
                return if t_1 < t_3 { 
                    Some(Square { first: t_1.clone(), second: t_3.clone(), side: t_2.clone() }) 
                } else {
                    Some(Square { first: t_3.clone(), second: t_1.clone(), side: t_2.clone() }) 
                }
            } else {
                return None;
            }
        } else if t_1.x == t_3.x  {
            if t_2.y == t_1.y {
                return if t_2 < t_3 { 
                    Some(Square { first: t_2.clone(), second: t_3.clone(), side: t_1.clone() }) 
                } else {
                    Some(Square { first: t_3.clone(), second: t_2.clone(), side: t_1.clone() }) 
                }
            } else if t_2.y == t_3.y {
                return if t_1 < t_2 { 
                    Some(Square { first: t_1.clone(), second: t_2.clone(), side: t_3.clone() }) 
                } else {
                    Some(Square { first: t_2.clone(), second: t_1.clone(), side: t_3.clone() }) 
                }
            } else {
                return None;
            }
        } else if t_2.x == t_3.x {
            if t_1.y == t_2.y {
                return if t_1 < t_3 { 
                    Some(Square { first: t_1.clone(), second: t_3.clone(), side: t_2.clone() }) 
                } else {
                    Some(Square { first: t_3.clone(), second: t_1.clone(), side: t_2.clone() }) 
                }
            } else if t_1.y == t_3.y {
                return if t_1 < t_2 { 
                    Some(Square { first: t_1.clone(), second: t_2.clone(), side: t_3.clone() }) 
                } else {
                    Some(Square { first: t_2.clone(), second: t_1.clone(), side: t_3.clone() }) 
                }
            } else {
                return None;
            }
        } else { 
            return None;
        }
    }

    fn size(&self) -> i64 {
        self.first.area(&self.second)
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

        let m = Matrix::from(&tiles);

        let squares: HashSet<Square> = tiles.iter()
            .enumerate()
            .flat_map(|(i_1, t_1)| 
                tiles[i_1+1..].iter().enumerate().flat_map(|(i_2, t_2)| 
                    tiles[i_2+1..].iter()
                        .map(|t_3| Square::create_square(t_1, t_2, t_3))
                        .filter(|s| s.is_some())
                        .map(|s| s.unwrap())
            )).collect();

        println!("Matrix:\n{}", m);
        println!("Squares:\n{:?}", squares);
        println!("len square: {}", squares.len());

        let result = squares.iter()
            .max_by_key(|s| s.size()).expect("No max found for square");
        println!("Found {:?}", result);
        Ok(result.size())
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
}