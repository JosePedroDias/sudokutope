use rustsolver::{solve40, solve60};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No arguments: run default test cases
            run_default_tests();
        }
        2 => {
            // Single argument: generate random puzzle of size N
            let size: usize = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: First argument must be a number (40 or 60)");
                    process::exit(1);
                }
            };

            if size != 40 && size != 60 {
                eprintln!("Error: Size must be either 40 or 60");
                process::exit(1);
            }

            generate_random_puzzle(size);
        }
        3 => {
            // Two arguments: solve puzzle with given input
            let size: usize = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: First argument must be a number (40 or 60)");
                    process::exit(1);
                }
            };

            if size != 40 && size != 60 {
                eprintln!("Error: Size must be either 40 or 60");
                process::exit(1);
            }

            let input = &args[2];
            solve_puzzle(size, input);
        }
        _ => {
            eprintln!("Error: Too many arguments");
            eprintln!("Usage:");
            eprintln!("  {} [no args]           - Run default test cases", args[0]);
            eprintln!("  {} <40|60>             - Generate random puzzle", args[0]);
            eprintln!("  {} <40|60> <json>      - Solve given puzzle", args[0]);
            process::exit(1);
        }
    }
}

fn run_default_tests() {
    // Test with the easy puzzle from the JavaScript file
    let input_40 = r#"[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]"#;

    println!("Solving 40-cell puzzle...");
    let result = solve40(input_40);
    println!("Result: {}", result);

    println!("\n---\n");

    // Test with the medium puzzle from the JavaScript file
    let input_60 = r#"[null,null,null,null,null,3,null,1,null,null,null,null,null,null,null,null,5,2,null,8,null,null,null,3,null,null,null,null,7,null,6,9,null,null,null,null,null,null,null,2,null,null,9,null,null,null,4,null,null,null,null,null,null,null,null,null,null,null,null,null]"#;

    println!("Solving 60-cell puzzle...");
    let result = solve60(input_60);
    println!("Result: {}", result);
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

