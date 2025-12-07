use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::{fmt, thread, time};

use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day7;

#[derive(PartialEq)]
enum Cell {
    Start,
    Dot,
    Splitter
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'S' => Cell::Start,
            '.' => Cell::Dot,
            '^' => Cell::Splitter,
            e => panic!("This character is not ok {e}")
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Start => write!(f, "S"),
            Cell::Dot => write!(f, "."),
            Cell::Splitter => write!(f, "^")
        }
    }
}

struct Puzzle {
    m: Vec<Vec<Cell>>,
    h: usize,
    w: usize,
    all_beams: HashMap<(usize, usize), usize>,
    cur_beams: HashSet<(usize, usize)>,
    iter: usize,
    res: i64
}

impl From<String> for Puzzle {
    fn from(value: String) -> Self {
        let matrix: Vec<Vec<Cell>> = value.split('\n')
            .filter(|row| !row.trim().is_empty())
            .map(|row| row.chars().map(Cell::from).collect::<Vec<Cell>>())
            .collect();
        let h = matrix.len();
        let w = matrix[0].len();

        let start = matrix.iter()
            .enumerate()
            .map(|(i, row)| (
                i,
                row.iter().enumerate().find(|(_, c)| Cell::Start == **c)
            ))
            .find(|(_, found_row)| found_row.is_some())
            .map(|(i, c)| (i, c.unwrap().0))
            .expect("There must be a start");

        let mut all_beams: HashMap<(usize, usize), usize> = HashMap::new();
        all_beams.insert(start, 1);
        let mut cur_beams: HashSet<(usize, usize)> = HashSet::new();
        cur_beams.insert(start);

        Puzzle { m: matrix, all_beams, cur_beams, res: 0, h, w, iter: 0 }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "It: {}, res: {}", self.iter, self.res)?;

        for i in 0..self.h {
            for j in 0..self.w {
                let c = &self.m[i][j];
                if let Cell::Start = c {
                    write!(f, " {c}")?;
                } else if let Some(nb_beam) = self.all_beams.get(&(i, j)) {
                    write!(f, "{}|", nb_beam)?;
                } else {
                    write!(f, " {c}")?;
                }
            }
            writeln!(f, "")?;
        }
        
        let last_beams: Vec<(&(usize, usize), &usize)> = self.all_beams.iter()
            .filter(|(beam, _nb_beam)| beam.0 == self.iter)
            .collect();
        writeln!(f, "Nb beam last layer: {:?}", last_beams)?;

        Ok(())
    }
}

impl Puzzle {
    fn step(&mut self) {
        self.iter += 1;
        let mut new_beams: HashSet<(usize, usize)> = HashSet::new();
        for beam in self.cur_beams.iter().filter(|(i, _)| *i != self.h - 1) {
            let nb_beams = self.all_beams[beam];
            if let Cell::Splitter = &self.m[beam.0 + 1][beam.1] {
                self.res += 1;
                if 1 <= beam.1 {
                    let new_beam = (beam.0 + 1, beam.1 - 1);
                    new_beams.insert(new_beam);
                    self.all_beams.entry(new_beam)
                        .and_modify(|existing_beams| { *existing_beams += nb_beams })
                        .or_insert(nb_beams);
                }

                if beam.1 + 1 < self.w {
                    let new_beam = (beam.0 + 1, beam.1 + 1);
                    new_beams.insert(new_beam);
                    self.all_beams.entry(new_beam)
                        .and_modify(|existing_beams| { *existing_beams += nb_beams })
                        .or_insert(nb_beams);
                }
            } else {
                let new_beam = (beam.0 + 1, beam.1);
                new_beams.insert(new_beam);
                self.all_beams.entry(new_beam)
                    .and_modify(|existing_beams| { *existing_beams += nb_beams })
                    .or_insert(nb_beams);
            }
        }

        self.cur_beams = new_beams;
    }
}

impl Day for Day7 {
    fn get_nb(&self) -> i8 {
        7
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let mut p = Puzzle::from(input);
        // println!("Puzzle start:\n{p}");

        while !p.cur_beams.is_empty() {
            p.step();
            // Debug purposes thread::sleep(time::Duration::from_millis(70));
            // Reset the cursor
            // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            // println!("{p}");
        }
        
        Ok(p.res)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let mut p = Puzzle::from(input);

        while !p.cur_beams.is_empty() {
            p.step();
        }

        let result: usize = p.all_beams.iter()
            .filter(|(beam, _)| beam.0 == p.iter - 1)
            .map(|(_, nb_beam)| nb_beam)
            .sum();
        
        Ok(result as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day7::Day7;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
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
        let result = Day7.part1(test_input).expect("There should be a result");

        assert_eq!(result, 21);
    }

    #[test]
    fn it_test_example_part2_simple() {
        let test_input = 
".......S.......
...............
.......^.......
...............".to_string();
        let result = Day7.part2(test_input).expect("There should be a result");

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
        let result = Day7.part2(test_input).expect("There should be a result");

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
        let result = Day7.part2(test_input).expect("There should be a result");

        assert_eq!(result, 40);
    }
}