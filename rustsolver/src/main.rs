use rustsolver::{add_gaps, assess_difficulty, solve40, solve60};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Command required");
        print_usage(&args[0]);
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "solve" => {
            if args.len() < 3 {
                eprintln!("Error: 'solve' requires a size argument (40 or 60)");
                print_usage(&args[0]);
                process::exit(1);
            }

            let size: usize = match args[2].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: Size must be a number (40 or 60)");
                    process::exit(1);
                }
            };

            if size != 40 && size != 60 {
                eprintln!("Error: Size must be either 40 or 60");
                process::exit(1);
            }

            if args.len() == 3 {
                // No input provided, generate random puzzle
                generate_random_puzzle(size);
            } else if args.len() == 4 {
                // Input provided, solve it
                let input = &args[3];
                solve_puzzle(size, input);
            } else {
                eprintln!("Error: Too many arguments for 'solve'");
                print_usage(&args[0]);
                process::exit(1);
            }
        }
        "add_gaps" => {
            if args.len() < 4 || args.len() > 5 {
                eprintln!("Error: 'add_gaps' requires 2-3 arguments: <json_string> <gap_count> [num_puzzles]");
                print_usage(&args[0]);
                process::exit(1);
            }

            let json_input = &args[2];
            let gap_count: usize = match args[3].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: gap_count must be a number");
                    process::exit(1);
                }
            };

            let num_puzzles: usize = if args.len() == 5 {
                match args[4].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Error: num_puzzles must be a number");
                        process::exit(1);
                    }
                }
            } else {
                1 // Default to 1 puzzle
            };

            let result = add_gaps(json_input, gap_count, num_puzzles);
            println!("{}", result);
        }
        "difficulty" => {
            if args.len() != 3 {
                eprintln!("Error: 'difficulty' requires exactly 1 argument: <json_string>");
                print_usage(&args[0]);
                process::exit(1);
            }

            let json_input = &args[2];
            let result = assess_difficulty(json_input);
            println!("{}", result);
        }
        _ => {
            eprintln!("Error: Unknown command '{}'. Must be one of: solve, add_gaps, difficulty", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  {} <command> [arguments...]", program_name);
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  solve <40|60>                             - Generate random puzzle");
    eprintln!("  solve <40|60> <json>                      - Solve given puzzle");
    eprintln!("  add_gaps <json> <gaps> [num_puzzles]      - Add gaps to solved puzzle");
    eprintln!("  difficulty <json>                         - Assess puzzle difficulty");
    eprintln!();
    eprintln!("The add_gaps command:");
    eprintln!("  - Takes a solved puzzle and adds gaps incrementally");
    eprintln!("  - Shows difficulty metrics at each step (printed to stderr)");
    eprintln!("  - Can generate multiple puzzle variations with [num_puzzles] (default: 1)");
    eprintln!("  - Returns JSON array of generated puzzles");
    eprintln!();
    eprintln!("The difficulty command:");
    eprintln!("  - Takes any puzzle (with or without gaps)");
    eprintln!("  - Eliminates options that conflict with fixed values");
    eprintln!("  - Returns total number of remaining options");
    eprintln!("  - Higher numbers = more difficult puzzles");
}

fn generate_random_puzzle(size: usize) {
    let null_array = format!("[{}]", "null,".repeat(size - 1) + "null");

    println!("Generating random {}-cell puzzle...", size);
    let result = if size == 40 {
        solve40(&null_array)
    } else {
        solve60(&null_array)
    };

    println!("{}", result);
}

fn solve_puzzle(size: usize, input: &str) {
    println!("Solving {}-cell puzzle...", size);
    let result = if size == 40 {
        solve40(input)
    } else {
        solve60(input)
    };

    println!("{}", result);
}

