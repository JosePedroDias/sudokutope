# Rust Sudokutope Solver

A high-performance Rust implementation of the Sudokutope puzzle solver.

## Features

- Solves 40-cell and 60-cell Sudokutope puzzles
- Constraint propagation with early termination on invalid states
- Backtracking search with heuristics
- JSON input/output format

## Building

```bash
cargo build --release
```

## Usage

### As a Library

```rust
use rustsolver::{solve40, solve60};

// Solve a 40-cell puzzle
let input = r#"[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]"#;
let result = solve40(input);
println!("Solution: {}", result);

// Solve a 60-cell puzzle
let input = r#"[null,null,null,null,null,3,null,1,null,null,null,null,null,null,null,null,5,2,null,8,null,null,null,3,null,null,null,null,7,null,6,9,null,null,null,null,null,null,null,2,null,null,9,null,null,null,4,null,null,null,null,null,null,null,null,null,null,null,null,null]"#;
let result = solve60(input);
println!("Solution: {}", result);
```

### As a Binary

The binary supports three modes:

**1. No arguments - Run default test cases:**
```bash
cargo run --release
# or
./target/release/rustsolver
```

**2. Single argument - Generate a random puzzle:**
```bash
cargo run --release -- 40    # Generate random 40-cell puzzle
cargo run --release -- 60    # Generate random 60-cell puzzle
# or
./target/release/rustsolver 40
./target/release/rustsolver 60
```

**3. Two arguments - Solve a specific puzzle:**
```bash
cargo run --release -- 40 '[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]'
# or
./target/release/rustsolver 40 '[1,null,null,7,...]'
```

## API

### `solve40(json_input: &str) -> String`

Solves a 40-cell Sudokutope puzzle.

- **Input**: JSON string representing an array of 40 elements, where `null` represents an empty cell and numbers represent filled cells (1-8)
- **Output**: JSON string representing the solution array, or an error object `{"error": "message"}`

### `solve60(json_input: &str) -> String`

Solves a 60-cell Sudokutope puzzle.

- **Input**: JSON string representing an array of 60 elements, where `null` represents an empty cell and numbers represent filled cells (0-9)
- **Output**: JSON string representing the solution array, or an error object `{"error": "message"}`

## Algorithm

The solver uses:

1. **Constraint propagation**: Eliminates invalid options from cells based on the puzzle constraints
2. **Early termination**: Stops propagation immediately when an invalid state is detected (cell with 0 valid options)
3. **Backtracking search**: Tries different values for cells with the fewest options first
4. **Heuristics**: Avoids recently chosen cells to improve search diversity

## Performance

The Rust implementation includes optimizations for early detection of dead-end branches, significantly reducing wasted computation compared to naive backtracking approaches.

### Performance Monitoring

The solver automatically reports:

1. **Solve time**: Displayed in seconds after each solve completes
   ```
   Solve time: 0.141 seconds
   ```

2. **Progress updates**: Progress is reported every second, showing filled cells at the time
   ```
   Progress: 35/60 cells filled
   Progress: 42/60 cells filled
   Progress: 58/60 cells filled
   ```

These metrics help you understand solver performance and track progress on difficult puzzles.

