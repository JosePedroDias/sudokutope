use rand::seq::SliceRandom;
use serde_json::Value;
use std::collections::HashSet;

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

fn solve_step<const N: usize>(
    max_target_size: usize,
    constraints: &[[usize; N]],
    state: &mut State,
    last_choices: &mut Vec<usize>,
    total_steps: &mut usize,
) -> Option<State> {
    *total_steps += 1;

    if is_solved(state) {
        println!("solved after {} steps!", total_steps);
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

    // Try each option with backtracking
    for option in options {
        // Create a deep copy of the state for this branch
        let mut new_state = deep_copy_state(state);
        new_state[idx] = Cell::Fixed(option);

        // Check if this choice is immediately invalid
        if !is_valid(&new_state, constraints) {
            continue;
        }

        // Propagate constraints: eliminate invalid options from other cells
        let mut changed = true;
        let mut has_invalid_cell = false;
        while changed && !has_invalid_cell {
            changed = false;
            for i in 0..new_state.len() {
                if matches!(new_state[i], Cell::Fixed(_)) {
                    continue;
                }

                let options_to_check: Vec<u8> = if let Cell::Options(opts) = &new_state[i] {
                    opts.iter().copied().collect()
                } else {
                    continue;
                };

                for opt in options_to_check {
                    let mut test_state = new_state.clone();
                    test_state[i] = Cell::Fixed(opt);
                    if !is_valid(&test_state, constraints) {
                        if let Cell::Options(opts) = &mut new_state[i] {
                            opts.remove(&opt);
                            changed = true;
                        }
                    }
                }

                // Convert to number if only one option left
                if let Cell::Options(opts) = &new_state[i] {
                    if opts.len() == 1 {
                        let val = *opts.iter().next().unwrap();
                        new_state[i] = Cell::Fixed(val);
                        changed = true;
                    } else if opts.is_empty() {
                        // This branch is invalid - exit immediately
                        has_invalid_cell = true;
                        break;
                    }
                }
            }
        }

        if has_invalid_cell {
            continue;
        }

        // Recursively solve with this choice
        last_choices.push(idx);
        if last_choices.len() > 10 {
            last_choices.remove(0);
        }

        let result = solve_step(max_target_size, constraints, &mut new_state, last_choices, total_steps);

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

    match solve_step(8, &CONSTRAINTS_40, &mut state, &mut last_choices, &mut total_steps) {
        Some(solution) => state_to_json(&solution),
        None => r#"{"error":"No solution found"}"#.to_string(),
    }
}

pub fn solve60(json_input: &str) -> String {
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

    match solve_step(10, &CONSTRAINTS_60, &mut state, &mut last_choices, &mut total_steps) {
        Some(solution) => state_to_json(&solution),
        None => r#"{"error":"No solution found"}"#.to_string(),
    }
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
}
