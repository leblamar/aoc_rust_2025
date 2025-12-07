use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day6;

#[derive(Debug)]
enum Op {
    Plus,
    Mult
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "*" => Op::Mult,
            "+" => Op::Plus,
            o => panic!("Unknown operator: {o}")
        }
    }
}

struct ParseError {}

impl TryFrom<char> for Op {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Op::Mult),
            '+' => Ok(Op::Plus),
            _ => Err(ParseError {})
        }
    }
}

impl Op {
    fn apply(&self, values: Vec<u128>) -> u128 {
        match self {
            Op::Mult => values.iter().fold(1, |acc, val| acc * val),
            Op::Plus => values.iter().sum()
        }
    }
}

impl Day for Day6 {
    fn get_nb(&self) -> i8 {
        6
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let grid: Vec<Vec<&str>> = input.split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.split(' ').map(|g| g.trim()).filter(|e| !e.is_empty()).collect::<Vec<&str>>())
            .collect();
        let h = grid.len();
        let w = grid[0].len();
        if grid.iter().any(|line| line.len() != w) {
            panic!("It should not be here. It means that not all the lines have same length");
        }

        let mut result = 0;
        for j in 0..w {
            let cur_op = Op::from(grid[h-1][j]);
            let values: Vec<u128> = grid[..h-1].iter()
                .map(|row| row[j])
                .map(|val| val.parse().expect("Parse should not failed"))
                .collect();

            result += cur_op.apply(values);
        }

        Ok(result as i64)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let grid: Vec<Vec<char>> = input.split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let h = grid.len();
        let w = grid[0].len();
        if grid.iter().any(|line| line.len() != w) {
            panic!("It should not be here. It means that not all the lines have same length");
        }

        let mut result = 0;
        let mut last_op = Op::Mult;
        let mut values_acc: Vec<u128> = vec![];
        for j in 0..w {
            let cur_op_res = Op::try_from(grid[h-1][j]);
            if let Ok(cur_op) = cur_op_res {
                last_op = cur_op;
            }
            

            let value_opt: Option<u128> = grid[..h-1].iter()
                .map(|row| row[j])
                .map(|val| val.to_digit(10))
                .fold(
                    None, 
                    |acc_opt, val_opt| 
                        if let Some(val) = val_opt {
                            if let Some(acc) = acc_opt { Some(acc * 10 + val as u128)} else { Some(val as u128) }
                        } else { acc_opt }
                );
            if let Some(value) = value_opt {
                values_acc.push(value);
            } else {
                result += last_op.apply(values_acc);
                values_acc = vec![];
            }
        }

        result += last_op.apply(values_acc);

        Ok(result as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day6::Day6;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
        let test_input = 
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ".to_string();
        let result = Day6.part1(test_input).expect("There should be a result");

        assert_eq!(result, 4277556);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = 
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ".to_string();
        let result = Day6.part2(test_input).expect("There should be a result");

        assert_eq!(result, 3263827);
    }
}