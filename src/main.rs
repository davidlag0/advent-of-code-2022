use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

mod day1;
mod day2;

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

fn load_file(filename: PathBuf) -> std::string::String {
    fs::read_to_string(filename).unwrap()
}

type SolverFn = fn(&str) -> Result<i64, &'static str>;

fn solve(day: u8, part: u8) -> Result<SolverFn, (u8, u8)> {
    match (day, part) {
        (1, 1) => Ok(day1::part1),
        (1, 2) => Ok(day1::part2),
        (2, 1) => Ok(day2::part1),
        (2, 2) => Ok(day2::part2),
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

    match solve(cli.day, cli.part) {
        Ok(solve_function) => {
            chrono_start = Instant::now();
            solution_result = solve_function(&load_file(day_input));
            chrono_stop = chrono_start.elapsed().as_micros();
            total_time += chrono_stop;

            match solution_result {
                Ok(solution) => println!(
                    "Solution of Day {}, Part {}: {:?}, Time: {}μs",
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
