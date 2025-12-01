use std::env;

pub mod days;
pub mod utils;
pub mod day_factory;

#[tokio::main]
async fn main() {
    println!("Let's start Advent of Code 2025!!!");

    let args: Vec<String> = env::args().collect();
    let nb_args = args.len();

    if nb_args <= 0 && 3 < nb_args {
        println!("You need to add args, first arg is for day, second one is for part. Ex: cargo run 4 2 will run the part 2 of the day 4.");
        println!("If you don't add any part, then it will run all of them. And without days it will run all of them.");
        return;
    }

    
    if nb_args == 1 {
        run_all_days().await;
        return;
    } else if 2 == nb_args {
        let day_str = &args[1];
        let day_result = day_str.parse::<i8>();
        if let Ok(day) = day_result {
            run_all_parts(day).await;   
        } else {
            panic!("You did'nt passed a correct day: {day_str}.")
        }
    } else if 3 == nb_args {
        let day_str = &args[1];
        let day_result = day_str.parse::<i8>();
        let part_str = &args[2];
        let part_result = part_str.parse::<i8>();
        if let (Ok(day), Ok(part)) = (day_result, part_result) {
            run_given_part(day, part).await;
        } else {
            panic!("You did'nt passed a correct day: {day_str}. Or a correct part: {part_str}")
        }
    } else {
        panic!("We should never come here!")
    }
}

async fn run_all_days() {
    println!("Run all days!!!");
    for i in 1..=1 {
        run_all_parts(i).await;
    }
}

async fn run_all_parts(day_nb: i8) {
    println!("Start of day{day_nb}");
    
    let input_result = utils::fetch_input::get_input_data(day_nb).await;
    if let Err(error) = input_result {
        panic!("An error occured while getting the input data: {error}");
    }
    let input = input_result.unwrap();

    if let Some(day) = day_factory::create_day(day_nb) {
        println!("Start of part1!");
        match day.part1(input.clone()) {
            Ok(result) => println!("Part 1 result found: {result}"),
            Err(error) => println!("Error during part 1: {0}", error.message),
        }
        println!("Start of part2!");
        match day.part2(input) {
            Ok(result) => println!("Part 2 result found: {result}"),
            Err(error) => println!("Error during part 2: {0}", error.message),
        }
    }
}

async fn run_given_part(day_nb: i8, part: i8) {
    if part != 1 && part != 2 {
        panic!("The part must be 1 or 2, not {part}")
    }
    println!("Start of day{day_nb} part{part}!");

    let input_result = utils::fetch_input::get_input_data(day_nb).await;
    if let Err(error) = input_result {
        panic!("An error occured while getting the input data: {error}");
    }
    let input = input_result.unwrap();

    if let Some(day) = day_factory::create_day(day_nb) {
        let part_result = if part == 1 { day.part1(input) } else { day.part2(input) };
        println!("Start of part{part}!");
        match part_result {
            Ok(result) => println!("Part {part} result found: {result}"),
            Err(error) => println!("Error during part {part}: {0}", error.message),
        }
    }
}
