use std::env;

pub mod day1;

fn main() {
    println!("Let's start Advent of Code 2025!!!");

    let args: Vec<String> = env::args().collect();
    let nb_args = args.len();

    if nb_args <= 0 && 3 < nb_args {
        println!("You need to add args, first arg is for day, second one is for part. Ex: cargo run 4 2 will run the part 2 of the day 4.");
        println!("If you don't add any part, then it will run all of them. And without days it will run all of them.");
        return;
    }

    
    if nb_args == 1 {
        run_all_days();
        return;
    } else if 2 == nb_args {
        let day_str = &args[1];
        let day_result = day_str.parse::<i8>();
        if let Ok(day) = day_result {
            run_all_parts(day);   
        } else {
            panic!("You did'nt passed a correct day: {day_str}.")
        }
    } else if 3 == nb_args {
        let day_str = &args[1];
        let day_result = day_str.parse::<i8>();
        let part_str = &args[2];
        let part_result = part_str.parse::<i8>();
        if let (Ok(day), Ok(part)) = (day_result, part_result) {
            run_given_part(day, part);
        } else {
            panic!("You did'nt passed a correct day: {day_str}. Or a correct part: {part_str}")
        }
    } else {
        panic!("We should never come here!")
    }
}

fn run_all_days() {
    day1::part1::main();
}

fn run_all_parts(day: i8) {
    println!("All the parts of day{day} will be ran!");
    println!("Start of part1:")

}

fn run_given_part(day: i8, part: i8) {
    println!("Only part{part} of day{day} will be ran!");
    println!("Start of part{part}!");
}
