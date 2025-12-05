use crate::utils::day::Day;
use crate::utils::day_error::DayError;

pub struct Day4;

#[derive(Debug)]
enum Case {
    Dot,
    Roll
}

impl From<char> for Case {
    fn from(value: char) -> Self {
        match value {
            '.' => Case::Dot,
            '@' => Case::Roll,
            e => panic!("This character is not authorized: {e}")
        }
    }
}

fn get_directions() -> Vec<(i32, i32)> {
    vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
}

fn is_roll_ok(matrix: &Vec<Vec<Case>>, directions: &Vec<(i32, i32)>, i: usize, j: usize) -> bool {
    let mut is_ok = true;
    let mut nb_rolls = 0;
    for (d_i, d_j) in directions.iter() {
        let new_i = (i as i32 + d_i) as usize;
        let new_j = (j as i32 + d_j) as usize;

        if let Some(row) = matrix.get(new_i) {
            if let Some(Case::Roll) = row.get(new_j) {
                nb_rolls += 1;
            }
        }

        if nb_rolls == 4 {
            is_ok = false;
            break;
        }
    }

    is_ok
}

fn get_accessible_rolls(matrix: &Vec<Vec<Case>>) -> Vec<(usize, usize)> {
    let directions = get_directions();

    let mut accessible_rolls: Vec<(usize, usize)> = vec![];
    for (i, row) in matrix.iter().enumerate() {
        for (j, case) in row.iter().enumerate() {
            if let Case::Dot = case {
                continue;
            }

            if is_roll_ok(&matrix, &directions, i, j) {
                accessible_rolls.push((i, j));
            }
        }
    }

    accessible_rolls
}

fn update_matrix(matrix: &mut Vec<Vec<Case>>, accessible_rolls: &Vec<(usize, usize)>) {
    for (roll_i, roll_j) in accessible_rolls {
        matrix[*roll_i][*roll_j] = Case::Dot;
    }
}

impl Day for Day4 {
    fn get_nb(&self) -> i8 {
        4
    }

    fn part1(&self, input: String) -> Result<i64, DayError<'_>> {
        let matrix = input.split('\n')
            .filter(|row| row.trim() != "")
            .map(|row| row.chars().map(Case::from).collect::<Vec<Case>>())
            .collect::<Vec<Vec<Case>>>();

        let accessible_rolls = get_accessible_rolls(&matrix);
        let result = accessible_rolls.len();

        Ok(result as i64)
    }

    fn part2(&self, input: String) -> Result<i64, DayError<'_>> {
        let mut matrix = input.split('\n')
            .filter(|row| row.trim() != "")
            .map(|row| row.chars().map(Case::from).collect::<Vec<Case>>())
            .collect::<Vec<Vec<Case>>>();

        let mut accessible_rolls = get_accessible_rolls(&matrix);
        let mut result = accessible_rolls.len();
        while accessible_rolls.len() != 0 {
            update_matrix(&mut matrix, &accessible_rolls);
            accessible_rolls = get_accessible_rolls(&matrix);
            result += accessible_rolls.len();
        }

        Ok(result as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day4::Day4;
    use crate::utils::day::Day;

    #[test]
    fn it_test_example_part1() {
        let test_input = 
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.".to_string();
        let result = Day4.part1(test_input).expect("There should be a result");

        assert_eq!(result, 13);
    }

    #[test]
    fn it_test_example_part2() {
        let test_input = 
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.".to_string();
        let result = Day4.part2(test_input).expect("There should be a result");

        assert_eq!(result, 43);
    }
}