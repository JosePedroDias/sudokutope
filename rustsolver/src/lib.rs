use rand::seq::SliceRandom;
use rayon::prelude::*;
use serde_json::Value;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const CONSTRAINTS_40: [[usize; 8]; 15] = [
    [1, 32, 35, 14, 13, 33, 34, 6],
    [0, 26, 25, 4, 12, 24, 18, 5],
    [28, 20, 9, 29, 2, 8, 19, 27],
    [22, 10, 21, 23, 11, 31, 30, 39],
    [3, 37, 38, 17, 16, 15, 7, 36],
    [32, 33, 34, 35, 36, 37, 38, 39],
    [24, 12, 13, 14, 35, 15, 16, 17],
    [18, 25, 4, 5, 14, 34, 6, 7],
    [8, 19, 26, 0, 5, 13, 33, 1],
    [2, 9, 20, 27, 0, 4, 12, 32],
    [31, 30, 29, 28, 27, 26, 25, 24],
    [39, 23, 22, 21, 28, 20, 19, 18],
    [17, 38, 11, 10, 21, 29, 9, 8],
    [1, 6, 15, 36, 3, 11, 23, 31],
    [2, 30, 22, 10, 3, 37, 16, 7],
];

const CONSTRAINTS_60: [[usize; 10]; 18] = [
    [7, 15, 6, 1, 54, 53, 52, 51, 27, 26],
    [50, 25, 24, 40, 14, 13, 12, 41, 5, 4],
    [0, 44, 43, 42, 35, 34, 33, 32, 19, 18],
    [45, 20, 46, 36, 8, 9, 47, 37, 2, 48],
    [21, 10, 22, 11, 38, 23, 58, 49, 39, 59],
    [31, 57, 30, 17, 3, 56, 29, 16, 55, 28],
    [8, 9, 47, 37, 21, 10, 11, 57, 30, 17],
    [8, 19, 34, 43, 0, 5, 13, 25, 51, 1],
    [2, 48, 38, 22, 10, 3, 56, 29, 16, 7],
    [2, 9, 20, 35, 44, 0, 4, 12, 24, 50],
    [49, 39, 23, 11, 3, 55, 28, 15, 6, 1],
    [49, 48, 47, 46, 45, 44, 43, 42, 41, 40],
    [59, 58, 57, 56, 55, 54, 53, 52, 51, 50],
    [59, 39, 38, 37, 36, 45, 35, 34, 33, 32],
    [31, 30, 29, 28, 54, 27, 26, 25, 24, 40],
    [31, 58, 23, 22, 21, 36, 46, 20, 19, 18],
    [17, 16, 15, 53, 27, 14, 13, 12, 41, 32],
    [7, 6, 52, 26, 14, 5, 4, 42, 33, 18],
];

#[derive(Clone, Debug)]
enum Cell {
    Fixed(u8),
    Options(HashSet<u8>),
}

type State = Vec<Cell>;

struct ProgressTracker {
    total_cells: usize,
    best_filled: Arc<AtomicUsize>,
    last_report: Arc<Mutex<Instant>>,
    solution_found: Arc<AtomicBool>,
}

impl ProgressTracker {
    fn new(total_cells: usize) -> Self {
        Self {
            total_cells,
            best_filled: Arc::new(AtomicUsize::new(0)),
            last_report: Arc::new(Mutex::new(Instant::now())),
            solution_found: Arc::new(AtomicBool::new(false)),
        }
    }

    fn is_solution_found(&self) -> bool {
        self.solution_found.load(Ordering::Relaxed)
    }

    fn mark_solution_found(&self) {
        self.solution_found.store(true, Ordering::Relaxed);
    }

    fn update(&self, state: &State) {
        let filled = state.iter().filter(|c| matches!(c, Cell::Fixed(_))).count();

        // Update best filled count
        let mut current_best = self.best_filled.load(Ordering::Relaxed);
        while filled > current_best {
            match self.best_filled.compare_exchange_weak(
                current_best,
                filled,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_best = x,
            }
        }

        
        if let Ok(mut last_report) = self.last_report.try_lock() {
            // Report progress every 100ms
            // if last_report.elapsed() >= Duration::from_millis(100) {
            // Report progress every second
            if last_report.elapsed() >= Duration::from_secs(1) {
                eprintln!("Progress: {}/{} cells filled", filled, self.total_cells);
                *last_report = Instant::now();
            }
        }
    }

    fn clone_refs(&self) -> Self {
        Self {
            total_cells: self.total_cells,
            best_filled: Arc::clone(&self.best_filled),
            last_report: Arc::clone(&self.last_report),
            solution_found: Arc::clone(&self.solution_found),
        }
    }
}

fn get_set_8() -> HashSet<u8> {
    (1..=8).collect()
}

fn get_set_10() -> HashSet<u8> {
    (0..=9).collect()
}

fn is_solved(state: &State) -> bool {
    state.iter().all(|cell| matches!(cell, Cell::Fixed(_)))
}

fn is_valid<const N: usize>(state: &State, constraints: &[[usize; N]]) -> bool {
    for constraint in constraints {
        let mut seen = HashSet::new();
        for &idx in constraint {
            if let Cell::Fixed(v) = state[idx] {
                if seen.contains(&v) {
                    return false;
                }
                seen.insert(v);
            }
        }
    }
    true
}

// Check if setting a specific cell to a value would be valid
fn is_valid_assignment<const N: usize>(
    state: &State,
    constraints: &[[usize; N]],
    cell_idx: usize,
    value: u8,
) -> bool {
    for constraint in constraints {
        // Only check constraints that include this cell
        if !constraint.contains(&cell_idx) {
            continue;
        }

        let mut seen = HashSet::new();
        for &idx in constraint {
            let v = if idx == cell_idx {
                value
            } else if let Cell::Fixed(val) = state[idx] {
                val
            } else {
                continue;
            };

            if seen.contains(&v) {
                return false;
            }
            seen.insert(v);
        }
    }
    true
}

fn deep_copy_state(state: &State) -> State {
    state
        .iter()
        .map(|cell| match cell {
            Cell::Fixed(v) => Cell::Fixed(*v),
            Cell::Options(opts) => Cell::Options(opts.clone()),
        })
        .collect()
}

fn find_indices_with_size(state: &State, target_size: usize, avoid_indices: &[usize]) -> Vec<usize> {
    state
        .iter()
        .enumerate()
        .filter(|(i, cell)| {
            !avoid_indices.contains(i)
                && matches!(cell, Cell::Options(opts) if opts.len() == target_size)
        })
        .map(|(i, _)| i)
        .collect()
}

fn find_indices_with_size_no_avoid(state: &State, target_size: usize) -> Vec<usize> {
    state
        .iter()
        .enumerate()
        .filter(|(_, cell)| matches!(cell, Cell::Options(opts) if opts.len() == target_size))
        .map(|(i, _)| i)
        .collect()
}

// Propagate constraints and return false if invalid state is reached
fn propagate_constraints<const N: usize>(state: &mut State, constraints: &[[usize; N]]) -> bool {
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..state.len() {
            if matches!(state[i], Cell::Fixed(_)) {
                continue;
            }

            let options_to_check: Vec<u8> = if let Cell::Options(opts) = &state[i] {
                opts.iter().copied().collect()
            } else {
                continue;
            };

            // Use optimized validation that doesn't clone the entire state
            for opt in options_to_check {
                if !is_valid_assignment(state, constraints, i, opt) {
                    if let Cell::Options(opts) = &mut state[i] {
                        opts.remove(&opt);
                        changed = true;
                    }
                }
            }

            // Convert to number if only one option left
            if let Cell::Options(opts) = &state[i] {
                if opts.len() == 1 {
                    let val = *opts.iter().next().unwrap();
                    state[i] = Cell::Fixed(val);
                    changed = true;
                } else if opts.is_empty() {
                    // This branch is invalid
                    return false;
                }
            }
        }
    }
    true
}

fn solve_step<const N: usize>(
    max_target_size: usize,
    constraints: &[[usize; N]],
    state: &mut State,
    last_choices: &mut Vec<usize>,
    total_steps: &mut usize,
    progress: &ProgressTracker,
) -> Option<State> {
    // Early exit if another thread already found a solution
    if progress.is_solution_found() {
        return None;
    }

    *total_steps += 1;

    // Update progress
    progress.update(state);

    if is_solved(state) {
        // Mark that we found a solution (this prevents other threads from continuing)
        progress.mark_solution_found();
        eprintln!("solved after {} steps!", total_steps);
        return Some(state.clone());
    }

    // Check if current state is already invalid (any cell has 0 options)
    for cell in state.iter() {
        if let Cell::Options(opts) = cell {
            if opts.is_empty() {
                return None;
            }
        }
    }

    let mut target_size = 2;
    let mut indices = Vec::new();

    // First try: avoid lastChoices
    while target_size <= max_target_size && indices.is_empty() {
        indices = find_indices_with_size(state, target_size, last_choices);
        if indices.is_empty() {
            target_size += 1;
        }
    }

    // Second try: if nothing found, ignore lastChoices constraint
    if indices.is_empty() {
        target_size = 2;
        while target_size <= max_target_size && indices.is_empty() {
            indices = find_indices_with_size_no_avoid(state, target_size);
            if indices.is_empty() {
                target_size += 1;
            }
        }
    }

    if indices.is_empty() {
        return None;
    }

    // Pick a random index from the candidates
    let mut rng = rand::thread_rng();
    let idx = *indices.choose(&mut rng).unwrap();

    let options: Vec<u8> = if let Cell::Options(opts) = &state[idx] {
        opts.iter().copied().collect()
    } else {
        return None;
    };

    // If we have multiple options and are early in the search, try parallel exploration
    // Only parallelize if we have 3+ options to avoid overhead
    if options.len() >= 3 && *total_steps < 100 {
        // Try options in parallel
        let progress_clone = progress.clone_refs();
        let result = options.par_iter().find_map_any(|&option| {
            let mut new_state = deep_copy_state(state);
            new_state[idx] = Cell::Fixed(option);

            if !is_valid(&new_state, constraints) {
                return None;
            }

            // Propagate constraints
            if !propagate_constraints(&mut new_state, constraints) {
                return None;
            }

            // Recursively solve with this choice
            let mut new_last_choices = last_choices.clone();
            new_last_choices.push(idx);
            if new_last_choices.len() > 10 {
                new_last_choices.remove(0);
            }

            let mut local_steps = *total_steps;
            solve_step(max_target_size, constraints, &mut new_state, &mut new_last_choices, &mut local_steps, &progress_clone)
        });

        if let Some(solution) = result {
            return Some(solution);
        }
        return None;
    }

    // Try each option with backtracking (sequential for small option sets or deep in search)
    for option in options {
        // Create a deep copy of the state for this branch
        let mut new_state = deep_copy_state(state);
        new_state[idx] = Cell::Fixed(option);

        // Check if this choice is immediately invalid
        if !is_valid(&new_state, constraints) {
            continue;
        }

        // Propagate constraints
        if !propagate_constraints(&mut new_state, constraints) {
            continue;
        }

        // Recursively solve with this choice
        last_choices.push(idx);
        if last_choices.len() > 10 {
            last_choices.remove(0);
        }

        let result = solve_step(max_target_size, constraints, &mut new_state, last_choices, total_steps, progress);

        // Remove the last choice for backtracking
        last_choices.pop();

        if result.is_some() {
            return result;
        }
    }

    None
}

fn parse_input(json_str: &str) -> Result<Vec<Option<u8>>, String> {
    let parsed: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let arr = parsed.as_array()
        .ok_or_else(|| "Input must be a JSON array".to_string())?;

    let mut result = Vec::new();
    for val in arr {
        if val.is_null() {
            result.push(None);
        } else if let Some(num) = val.as_u64() {
            result.push(Some(num as u8));
        } else {
            return Err("Array elements must be null or numbers".to_string());
        }
    }

    Ok(result)
}

fn state_to_json(state: &State) -> String {
    let values: Vec<u8> = state
        .iter()
        .map(|cell| match cell {
            Cell::Fixed(v) => *v,
            Cell::Options(_) => 0, // Should not happen in solved state
        })
        .collect();

    serde_json::to_string(&values).unwrap()
}

pub fn solve40(json_input: &str) -> String {
    let start_time = Instant::now();

    let input = match parse_input(json_input) {
        Ok(v) => v,
        Err(e) => return format!(r#"{{"error":"{}"}}"#, e),
    };

    if input.len() != 40 {
        return r#"{"error":"Input must have exactly 40 elements"}"#.to_string();
    }

    let mut state: State = input
        .into_iter()
        .map(|v| match v {
            Some(val) => Cell::Fixed(val),
            None => Cell::Options(get_set_8()),
        })
        .collect();

    let mut last_choices = Vec::new();
    let mut total_steps = 0;
    let progress = ProgressTracker::new(40);

    let result = match solve_step(8, &CONSTRAINTS_40, &mut state, &mut last_choices, &mut total_steps, &progress) {
        Some(solution) => state_to_json(&solution),
        None => r#"{"error":"No solution found"}"#.to_string(),
    };

    let elapsed = start_time.elapsed();
    eprintln!("Solve time: {:.3} seconds", elapsed.as_secs_f64());

    result
}

pub fn solve60(json_input: &str) -> String {
    let start_time = Instant::now();

    let input = match parse_input(json_input) {
        Ok(v) => v,
        Err(e) => return format!(r#"{{"error":"{}"}}"#, e),
    };

    if input.len() != 60 {
        return r#"{"error":"Input must have exactly 60 elements"}"#.to_string();
    }

    let mut state: State = input
        .into_iter()
        .map(|v| match v {
            Some(val) => Cell::Fixed(val),
            None => Cell::Options(get_set_10()),
        })
        .collect();

    let mut last_choices = Vec::new();
    let mut total_steps = 0;
    let progress = ProgressTracker::new(60);

    let result = match solve_step(10, &CONSTRAINTS_60, &mut state, &mut last_choices, &mut total_steps, &progress) {
        Some(solution) => state_to_json(&solution),
        None => r#"{"error":"No solution found"}"#.to_string(),
    };

    let elapsed = start_time.elapsed();
    eprintln!("Solve time: {:.3} seconds", elapsed.as_secs_f64());

    result
}

/// Add gaps to a solved puzzle incrementally, showing difficulty at each step
///
/// This function starts with a solved puzzle and adds gaps one by one,
/// printing the puzzle and its difficulty metrics after each gap is added.
/// This allows generating multiple puzzles with different difficulty levels
/// from a single solved puzzle.
///
/// # Arguments
/// * `json_input` - A JSON array string representing a SOLVED puzzle (no nulls)
/// * `gap_count` - Number of values to remove (replace with null)
/// * `num_puzzles` - Number of different puzzle variations to generate (default: 1)
///
/// # Returns
/// A JSON array of puzzles with their difficulty metrics
pub fn add_gaps(json_input: &str, gap_count: usize, num_puzzles: usize) -> String {
    let input = match parse_input(json_input) {
        Ok(v) => v,
        Err(e) => return format!(r#"{{"error":"{}"}}"#, e),
    };

    let puzzle_size = input.len();

    // Verify it's a solved puzzle (no nulls)
    if input.iter().any(|v| v.is_none()) {
        return r#"{"error":"Input must be a solved puzzle (no nulls)"}"#.to_string();
    }

    // Validate gap_count
    if gap_count > puzzle_size {
        return format!(r#"{{"error":"gap_count ({}) cannot exceed puzzle size ({})"}}"#, gap_count, puzzle_size);
    }

    if gap_count == 0 {
        return r#"{"error":"gap_count must be at least 1"}"#.to_string();
    }

    if num_puzzles == 0 {
        return r#"{"error":"num_puzzles must be at least 1"}"#.to_string();
    }

    // Determine puzzle type
    let puzzle_size_match = match puzzle_size {
        40 => 40,
        60 => 60,
        _ => return format!(r#"{{"error":"Puzzle must be 40 or 60 cells, got {}"}}"#, puzzle_size),
    };

    let mut all_puzzles = Vec::new();

    // Generate num_puzzles different variations
    for puzzle_num in 1..=num_puzzles {
        eprintln!("\n=== Puzzle {} ===", puzzle_num);

        // Create a shuffled list of indices to remove
        let mut indices: Vec<usize> = (0..puzzle_size).collect();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        // Incrementally add gaps and print progress
        for current_gaps in 1..=gap_count {
            let gaps_set: HashSet<usize> = indices.iter().take(current_gaps).copied().collect();

            // Build puzzle with current_gaps gaps
            let puzzle_values: Vec<Option<u8>> = input
                .iter()
                .enumerate()
                .map(|(i, &val)| {
                    if gaps_set.contains(&i) {
                        None
                    } else {
                        val
                    }
                })
                .collect();

            // Create state for difficulty assessment
            let mut state: State = puzzle_values
                .iter()
                .map(|&v| match v {
                    Some(val) => Cell::Fixed(val),
                    None => Cell::Options(if puzzle_size == 40 { get_set_8() } else { get_set_10() }),
                })
                .collect();

            // Propagate constraints
            let valid = match puzzle_size_match {
                40 => propagate_constraints(&mut state, &CONSTRAINTS_40),
                60 => propagate_constraints(&mut state, &CONSTRAINTS_60),
                _ => unreachable!(),
            };

            if !valid {
                eprintln!("Puzzle {} became invalid at {} gaps", puzzle_num, current_gaps);
                continue;
            }

            // Count complexity
            let (total_options, max_options, cells_with_options) = count_options(&state);

            if total_options == 0 && cells_with_options > 0 {
                eprintln!("Puzzle {} became unsolvable at {} gaps", puzzle_num, current_gaps);
                continue;
            }

            let avg_options = if cells_with_options > 0 {
                total_options as f64 / cells_with_options as f64
            } else {
                0.0
            };

            // Print puzzle as JSON array
            let puzzle_json: Vec<Value> = puzzle_values
                .iter()
                .map(|&v| match v {
                    Some(val) => Value::Number(val.into()),
                    None => Value::Null,
                })
                .collect();

            let puzzle_str = serde_json::to_string(&puzzle_json).unwrap();

            // Print to stderr so it doesn't interfere with final JSON output
            eprintln!("  Gaps: {:2} | Difficulty: {:3} total, {:4.2} avg, {:2} max | {}",
                      current_gaps, total_options, avg_options, max_options, puzzle_str);
        }

        // Add final puzzle to results
        let gaps_set: HashSet<usize> = indices.iter().take(gap_count).copied().collect();
        let final_puzzle: Vec<Value> = input
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                if gaps_set.contains(&i) {
                    Value::Null
                } else {
                    match val {
                        Some(v) => Value::Number(v.into()),
                        None => Value::Null,
                    }
                }
            })
            .collect();

        all_puzzles.push(serde_json::to_string(&final_puzzle).unwrap());
    }

    // Return array of all generated puzzles
    format!("[{}]", all_puzzles.join(","))
}

/// Helper function to count options in a state
/// Returns (total_options, max_options, cells_with_options)
fn count_options(state: &State) -> (usize, usize, usize) {
    let mut total_options = 0;
    let mut max_options = 0;
    let mut cells_with_options = 0;

    for cell in state {
        if let Cell::Options(opts) = cell {
            let count = opts.len();
            total_options += count;
            max_options = max_options.max(count);
            cells_with_options += 1;
        }
    }

    (total_options, max_options, cells_with_options)
}

/// Assess the difficulty of a puzzle by counting total options after basic validation
///
/// This function:
/// 1. Replaces any null with full options (8 or 10 depending on puzzle size)
/// 2. Eliminates options that would fail validation (conflict with fixed values)
/// 3. Returns the cumulative number of options left
///
/// # Arguments
/// * `json_input` - A JSON array string representing a puzzle (may contain nulls)
///
/// # Returns
/// A JSON string with the total number of options: `{"difficulty": <usize>}`
/// Higher numbers indicate more difficult puzzles
pub fn assess_difficulty(json_input: &str) -> String {
    let input = match parse_input(json_input) {
        Ok(v) => v,
        Err(e) => return format!(r#"{{"error":"{}"}}"#, e),
    };

    let puzzle_size = input.len();

    // Create initial state: replace nulls with full options
    let mut state: State = input
        .iter()
        .map(|&v| match v {
            Some(val) => Cell::Fixed(val),
            None => Cell::Options(if puzzle_size == 40 { get_set_8() } else { get_set_10() }),
        })
        .collect();

    // Eliminate options that would fail validation
    // Handle 40 and 60 cell puzzles separately due to different constraint types
    match puzzle_size {
        40 => {
            for constraint in &CONSTRAINTS_40 {
                // Collect fixed values in this constraint
                let fixed_values: HashSet<u8> = constraint
                    .iter()
                    .filter_map(|&idx| {
                        if let Cell::Fixed(val) = state[idx] {
                            Some(val)
                        } else {
                            None
                        }
                    })
                    .collect();

                // Remove fixed values from options in this constraint
                for &idx in constraint {
                    if let Cell::Options(ref mut opts) = state[idx] {
                        opts.retain(|&v| !fixed_values.contains(&v));
                    }
                }
            }
        }
        60 => {
            for constraint in &CONSTRAINTS_60 {
                // Collect fixed values in this constraint
                let fixed_values: HashSet<u8> = constraint
                    .iter()
                    .filter_map(|&idx| {
                        if let Cell::Fixed(val) = state[idx] {
                            Some(val)
                        } else {
                            None
                        }
                    })
                    .collect();

                // Remove fixed values from options in this constraint
                for &idx in constraint {
                    if let Cell::Options(ref mut opts) = state[idx] {
                        opts.retain(|&v| !fixed_values.contains(&v));
                    }
                }
            }
        }
        _ => return format!(r#"{{"error":"Puzzle must be 40 or 60 cells, got {}"}}"#, puzzle_size),
    }

    // Count total options
    let (total_options, _, _) = count_options(&state);

    format!(r#"{{"difficulty":{}}}"#, total_options)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve40_empty_puzzle() {
        let input = r#"[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null]"#;
        let result = solve40(input);

        // Empty puzzle has multiple valid solutions due to randomization
        // Just verify we got a valid solution (not an error)
        assert!(!result.contains("error"), "Expected a solution, got: {}", result);

        // Parse the result and verify it's a valid 40-element array
        let parsed: Result<Vec<u8>, _> = serde_json::from_str(&result);
        assert!(parsed.is_ok(), "Result should be valid JSON array");
        let solution = parsed.unwrap();
        assert_eq!(solution.len(), 40, "Solution should have 40 elements");

        // Verify all values are in range 1-8
        for &val in &solution {
            assert!(val >= 1 && val <= 8, "All values should be between 1 and 8");
        }

        // Verify the solution satisfies all constraints
        let state: State = solution.iter().map(|&v| Cell::Fixed(v)).collect();
        assert!(is_valid(&state, &CONSTRAINTS_40), "Solution should satisfy all constraints");
    }

    #[test]
    fn test_solve40_easy_puzzle() {
        let input = r#"[1,null,null,7,null,null,5,3,null,3,null,null,null,8,2,null,null,6,4,null,null,null,1,null,null,null,null,7,null,null,5,4,null,3,null,null,null,2,null,null]"#;
        let result = solve40(input);
        let expected = r#"[1,6,6,7,8,7,5,3,4,3,8,2,5,8,2,1,4,6,4,5,2,7,1,3,3,6,2,7,8,1,5,4,4,3,1,7,8,2,5,6]"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve60_medium_puzzle() {
        let input = r#"[null,null,null,null,null,3,null,1,null,null,null,null,null,null,null,null,5,2,null,8,null,null,null,3,null,null,null,null,7,null,6,9,null,null,null,null,null,null,null,2,null,null,9,null,null,null,4,null,null,null,null,null,null,null,null,null,null,null,null,null]"#;
        let result = solve60(input);
        let expected = r#"[0,6,2,0,7,3,8,1,9,8,4,5,6,4,0,9,5,2,2,8,1,0,7,3,5,1,4,3,7,8,6,9,1,6,7,4,5,3,9,2,2,8,9,5,3,0,4,7,6,1,9,2,5,7,0,4,3,1,6,8]"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve40_invalid_input() {
        let input = r#"[1,2,3]"#;
        let result = solve40(input);
        assert!(result.contains("error"));
    }

    #[test]
    fn test_solve60_invalid_input() {
        let input = r#"[1,2,3]"#;
        let result = solve60(input);
        assert!(result.contains("error"));
    }

    #[test]
    fn test_add_gaps() {
        let solved = r#"[1,6,6,7,8,7,5,3,4,3,8,2,5,8,2,1,4,6,4,5,2,7,1,3,3,6,2,7,8,1,5,4,4,3,1,7,8,2,5,6]"#;
        let result = add_gaps(solved, 15, 1);

        // Should not be an error
        assert!(!result.contains("error"), "Should not error: {}", result);

        // Parse as array of puzzles
        let parsed: Result<Vec<Vec<Option<u8>>>, _> = serde_json::from_str(&result);
        assert!(parsed.is_ok(), "Result should be valid JSON array: {}", result);

        let puzzles = parsed.unwrap();
        assert_eq!(puzzles.len(), 1, "Should have 1 puzzle");

        let puzzle = &puzzles[0];
        assert_eq!(puzzle.len(), 40, "Should have 40 elements");

        // Count nulls
        let null_count = puzzle.iter().filter(|v| v.is_none()).count();
        assert_eq!(null_count, 15, "Should have exactly 15 gaps");

        // Count non-nulls
        let filled_count = puzzle.iter().filter(|v| v.is_some()).count();
        assert_eq!(filled_count, 25, "Should have 25 filled cells");
    }

    #[test]
    fn test_add_gaps_multiple() {
        let solved = r#"[1,6,6,7,8,7,5,3,4,3,8,2,5,8,2,1,4,6,4,5,2,7,1,3,3,6,2,7,8,1,5,4,4,3,1,7,8,2,5,6]"#;
        let result = add_gaps(solved, 10, 3);

        // Parse as array of puzzles
        let parsed: Result<Vec<Vec<Option<u8>>>, _> = serde_json::from_str(&result);
        assert!(parsed.is_ok(), "Result should be valid JSON array");

        let puzzles = parsed.unwrap();
        assert_eq!(puzzles.len(), 3, "Should have 3 puzzles");

        // Each puzzle should have 10 gaps
        for puzzle in &puzzles {
            let null_count = puzzle.iter().filter(|v| v.is_none()).count();
            assert_eq!(null_count, 10, "Each puzzle should have exactly 10 gaps");
        }
    }

    #[test]
    fn test_add_gaps_invalid_count() {
        let solved = r#"[1,2,3,4,5]"#;
        let result = add_gaps(solved, 10, 1);
        assert!(result.contains("error"), "Should error when gap_count > puzzle size");
    }

    #[test]
    fn test_add_gaps_zero() {
        let solved = r#"[1,6,6,7,8,7,5,3,4,3,8,2,5,8,2,1,4,6,4,5,2,7,1,3,3,6,2,7,8,1,5,4,4,3,1,7,8,2,5,6]"#;
        let result = add_gaps(solved, 0, 1);
        assert!(result.contains("error"), "Should error when gap_count is 0");
    }

    #[test]
    fn test_assess_difficulty() {
        // Test with a solved puzzle (no gaps)
        let solved = r#"[1,6,6,7,8,7,5,3,4,3,8,2,5,8,2,1,4,6,4,5,2,7,1,3,3,6,2,7,8,1,5,4,4,3,1,7,8,2,5,6]"#;
        let result = assess_difficulty(solved);

        assert!(!result.contains("error"), "Should not error: {}", result);

        // Parse result
        let metrics: serde_json::Value = serde_json::from_str(&result).unwrap();

        // Solved puzzle should have 0 difficulty (no options)
        assert_eq!(metrics["difficulty"].as_u64().unwrap(), 0, "Solved puzzle should have 0 difficulty");
    }

    #[test]
    fn test_assess_difficulty_with_gaps() {
        // Test with a puzzle that has many gaps (should have remaining options)
        let puzzle = r#"[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null]"#;
        let result = assess_difficulty(puzzle);

        assert!(!result.contains("error"), "Should not error: {}", result);

        // Parse result
        let metrics: serde_json::Value = serde_json::from_str(&result).unwrap();

        // Empty puzzle should have very high difficulty (many options)
        let difficulty = metrics["difficulty"].as_u64().unwrap();
        assert!(difficulty > 100, "Empty puzzle should have high difficulty, got {}", difficulty);
    }

    #[test]
    fn test_assess_difficulty_invalid_puzzle() {
        let input = r#"[1,2,3]"#;
        let result = assess_difficulty(input);
        assert!(result.contains("error"), "Should error on invalid puzzle size");
    }
}
