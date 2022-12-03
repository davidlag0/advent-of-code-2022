use aoc_rust_2022::day1;
use aoc_rust_2022::day2;
use std::env;
use std::fs;
use std::process;
use std::time::Instant;

fn load_file(filename: &str) -> std::string::String {
    fs::read_to_string(filename).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut chrono_start;
    let mut chrono_stop;
    let mut solution_result;
    let mut total_time: u128 = 0;

    if args.len() < 2 {
        println!("not enough arguments");
        process::exit(1);
    }

    let base_path = args[1].clone();

    chrono_start = Instant::now();
    solution_result = day1::part1(&load_file(&[&base_path, "day1.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;

    match solution_result {
        Ok(solution) => println!(
            "Solution of Day 1, Part 1: {:?}, Time: {}μs",
            solution, chrono_stop
        ),
        Err(error) => println!(
            "A problem occured to solve the problem of Day 1, Part 1: {}, Time: {}μs",
            error, chrono_stop
        ),
    }

    chrono_start = Instant::now();
    solution_result = day1::part2(&load_file(&[&base_path, "day1.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;

    match solution_result {
        Ok(solution) => println!(
            "Solution of Day 1, Part 2: {:?}, Time: {}μs",
            solution, chrono_stop
        ),
        Err(error) => println!(
            "A problem occured to solve the problem of Day 1, Part 2: {}, Time: {}μs",
            error, chrono_stop
        ),
    }

    chrono_start = Instant::now();
    solution_result = day2::part1(&load_file(&[&base_path, "day2.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;

    match solution_result {
        Ok(solution) => println!(
            "Solution of Day 2, Part 1: {:?}, Time: {}μs",
            solution, chrono_stop
        ),
        Err(error) => println!(
            "A problem occured to solve the problem of Day 2, Part 1: {}, Time: {}μs",
            error, chrono_stop
        ),
    }

    chrono_start = Instant::now();
    solution_result = day2::part2(&load_file(&[&base_path, "day2.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;

    match solution_result {
        Ok(solution) => println!(
            "Solution of Day 2, Part 2: {:?}, Time: {}μs",
            solution, chrono_stop
        ),
        Err(error) => println!(
            "A problem occured to solve the problem of Day 2, Part 2: {}, Time: {}μs",
            error, chrono_stop
        ),
    }

    println!("\nTotal Time: {}μs", total_time);
}
