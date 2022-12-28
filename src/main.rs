use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(Parser)]
#[command(name = "advent-of-code-2022")]
#[command(about = "Calculate solutions for Advent of Code 2022 using Rust and the provided input", long_about = None)]
struct Cli {
    input_path: String,
    #[arg(short, long)]
    // TODO: Make this parameter optional and if not provided, solve all days.
    // day: Option<u8>
    day: u8,
    #[arg(short, long)]
    // TODO: Make this parameter optional and if not provided, solve all parts of a given day.
    // part: Option<u8>
    part: u8,
}

fn load_file(filename: PathBuf) -> Option<std::string::String> {
    let input_filename = filename.as_path().display().to_string();

    match fs::read_to_string(filename) {
        Ok(path) => Some(path),
        Err(err) => {
            println!("Could not load input file '{}'. {}", input_filename, err);
            None
        }
    }
}

type SolverFn = fn(&str) -> Result<String, String>;

fn solve(day: u8, part: u8) -> Result<SolverFn, (u8, u8)> {
    match (day, part) {
        (1, 1) => Ok(day1::part1),
        (1, 2) => Ok(day1::part2),
        (2, 1) => Ok(day2::part1),
        (2, 2) => Ok(day2::part2),
        (3, 1) => Ok(day3::part1),
        (3, 2) => Ok(day3::part2),
        (4, 1) => Ok(day4::part1),
        (4, 2) => Ok(day4::part2),
        (5, 1) => Ok(day5::part1),
        (5, 2) => Ok(day5::part2),
        (6, 1) => Ok(day6::part1),
        (6, 2) => Ok(day6::part2),
        (7, 1) => Ok(day7::part1),
        (7, 2) => Ok(day7::part2),
        (8, 1) => Ok(day8::part1),
        (8, 2) => Ok(day8::part2),
        (9, 1) => Ok(day9::part1),
        (9, 2) => Ok(day9::part2),
        (10, 1) => Ok(day10::part1),
        (10, 2) => Ok(day10::part2),
        (_, _) => Err((day, part)),
    }
}

fn main() {
    let cli = Cli::parse();

    let chrono_start;
    let chrono_stop;
    let solution_result;
    let mut total_time: u128 = 0;

    let day_input: PathBuf = [cli.input_path, format!("day{}.txt", cli.day)]
        .iter()
        .collect();

    if let Some(puzzle_input) = load_file(day_input) {
        match solve(cli.day, cli.part) {
            Ok(solve_function) => {
                chrono_start = Instant::now();
                solution_result = solve_function(&puzzle_input);
                chrono_stop = chrono_start.elapsed().as_micros();
                total_time += chrono_stop;

                match solution_result {
                    Ok(solution) => println!(
                        "Solution of Day {}, Part {}: {}, Time: {}μs",
                        cli.day, cli.part, solution, chrono_stop
                    ),
                    Err(error) => println!(
                        "A problem occured to solve the problem of Day {}, Part {}: {}, Time: {}μs",
                        cli.day, cli.part, error, chrono_stop
                    ),
                }
                println!("\nTotal Time: {}μs", total_time);
            }
            Err(_) => println!("Unsupported day {} and part {}", cli.day, cli.part),
        }
    }
}
