// Advent of Code 2024 - Day 6
// Problem link: https://adventofcode.com/2024/day/6
use std::sync::LazyLock;

pub fn solve() {
    println!("Solution for Day 6");
    let DAY = 6;
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    let input_file_path = exe_dir.join(format!("../../src/days/day{}/input.txt", DAY));

    let input_content = match std::fs::read_to_string(input_file_path) {
        Ok(content) => content,
        Err(e) => panic!("Something went wrong reading the input file: {}", e),
    };

    let test_input_file_path = exe_dir.join(format!("../../src/days/day{}/test_input.txt", DAY));

    let test_input_content = match std::fs::read_to_string(test_input_file_path) {
        Ok(content) => content,
        Err(e) => panic!("Something went wrong reading the test input file: {}", e),
    };

    let mut state = VisitationState::new();

    // Call solvePartOne with both inputs
    solvePartOne(&mut state, &input_content);

    // Call solvePartTwo with both inputs
    solvePartTwo(&state, &input_content);
}

fn solvePartOne(state: &mut VisitationState, input: &str) {
    println!("Part One:");
    println!("Input result: {}", calculate_part_one_result(state, input));
}

fn solvePartTwo(state: &VisitationState, input: &str) {
    println!("Part Two:");
    println!("Input result: {}", calculate_part_two_result(state, input));
}

<<<<<<< Updated upstream

fn get_next_coordinate(i: usize, j: usize, direction: char) -> Option<(usize, usize)> {
    match direction {
        '^' => if i > 0 { Some((i - 1, j)) } else { None },
        'v' => Some((i + 1, j)), // We will check bounds later
        '<' => if j > 0 { Some((i, j - 1)) } else { None },
        '>' => Some((i, j + 1)), // We will check bounds later
        _ => Some((i, j)), // No movement
    }
}
=======
>>>>>>> Stashed changes


struct VisitationState {
    visited_cells: std::collections::HashSet<(usize, usize)>,
}

impl VisitationState {
    fn new() -> Self {
        VisitationState {
            visited_cells: std::collections::HashSet::new(),
        }
    }

    fn mark_visited(&mut self, coordinates: (usize, usize)) {
        self.visited_cells.insert(coordinates);
    }

    fn has_visited(&self, coordinates: &(usize, usize)) -> bool {
        self.visited_cells.contains(coordinates)
    }
}


fn calculate_part_one_result(state: &mut VisitationState, input: &str) -> String {
    // Implement Part One calculation logic here
    let mut input_vec: Vec<Vec<char>> = input.lines().map(|line | line.chars().collect()).collect();

    let mut initial_i: usize = 0;
    let mut initial_j: usize = 0;

    //Find the initial position of the player
    for i in 0..input_vec.len() {
        for j in 0..input_vec[i].len() {
            if input_vec[i][j] == '^' || input_vec[i][j] == 'v' || input_vec[i][j] == '<' || input_vec[i][j] == '>' {
                initial_j = j;
                initial_i = i;
            }
        }
    }

    let mut i = initial_i;
    let mut j = initial_j;
    let mut direction = input_vec[i ][j];
    let mut total_steps = 0;

    while i >=0 && i < input_vec.len() && j >= 0 && j < input_vec[i].len() {

        if input_vec[i][j] != 'X' {
            input_vec[i][j] = 'X';
            state.mark_visited((i, j));
            total_steps += 1;
        }

        match get_next_coordinate(i, j, direction) {
            Some(next_coordinate) => {
                if next_coordinate.0 >= 0 && next_coordinate.0 < input_vec.len() && next_coordinate.1 >= 0 && next_coordinate.1 < input_vec[next_coordinate.0].len() {
                    if input_vec[next_coordinate.0][next_coordinate.1] == '#'{
                        direction = get_new_direction(direction);
                    } else {
                        i = next_coordinate.0;
                        j = next_coordinate.1;
                    }
                } else {
                    break;
                }
            },
            None => {
                // Handle the case when there's no valid next coordinate
                break;
            }
        }

    }

    return total_steps.to_string();
}


fn get_next_coordinate(i: usize, j: usize, direction: char) -> Option<(usize, usize)> {
    match direction {
        '^' => if i > 0 { Some((i - 1, j)) } else { None },
        'v' => Some((i + 1, j)), // We will check bounds later
        '<' => if j > 0 { Some((i, j - 1)) } else { None },
        '>' => Some((i, j + 1)), // We will check bounds later
        _ => Some((i, j)), // No movement
    }
}

fn get_new_direction(direction: char) -> char {
    match direction {
        '^' => '>',
        'v' => '<',
        '<' => '^',
        '>' => 'v',
        _ => direction,
    }
}

fn is_in_loop(input_vec: &Vec<Vec<char>>, start_i: usize, start_j: usize, start_direction: char) -> bool {
    let mut visited_cells: std::collections::HashSet<(usize, usize, char)> = std::collections::HashSet::new();

    let (mut i, mut j) = (start_i, start_j);
    let mut direction = start_direction;

    loop {
        // Check if we have looped back to a previously visited state
        if visited_cells.contains(&(i, j, direction)) {
            return true;
        }
        visited_cells.insert((i, j, direction));

        let mut tries = 0;
        let mut moved = false;

        // Try to move forward. If blocked, turn right until we find a free path or exhaust all possibilities
        while tries < 4 {
            if let Some((next_i, next_j)) = get_next_coordinate(i, j, direction) {
                // Check bounds
                if next_i >= input_vec.len() || next_j >= input_vec[next_i].len() {
                    // Out of bounds means no loop, just break out
                    return false;
                }

                // Check cell content
                if input_vec[next_i][next_j] == '#' {
                    // Turn right and try again
                    direction = get_new_direction(direction);
                } else {
                    // Free cell found, move forward
                    i = next_i;
                    j = next_j;
                    moved = true;
                    break;
                }
            } else {
                // No next coordinate means out of grid on top/left edge
                return false;
            }

            tries += 1;
        }

        if !moved {
            // Tried all four directions, no way forward => stuck, no loop
            return false;
        }
    }
}

<<<<<<< Updated upstream
fn calculate_part_two_result(state: &VisitationState, input: &str) -> String {
    //Loop through the visited cells and place a barrier there, then check if a loop is created or not and then revert back the barrier
=======

fn calculate_part_two_result(input: &str) -> String {
    // Implement Part Two calculation logic here
>>>>>>> Stashed changes
    let mut input_vec: Vec<Vec<char>> = input.lines().map(|line | line.chars().collect()).collect();
    //Loop through the visited cells and place a barrier there, then check if a loop is created or not and then revert back the barrier
    let mut total_steps = 0;

    //Get initial position and direction
    let mut initial_i = 0;
    let mut initial_j = 0;
    let mut initial_direction = '^';

    for i in 0..input_vec.len() {
        for j in 0..input_vec[i].len() {
            if input_vec[i][j] == '^' || input_vec[i][j] == 'v' || input_vec[i][j] == '<' || input_vec[i][j] == '>' {
                initial_j = j;
                initial_i = i;
                initial_direction = input_vec[i][j];
            }
        }
    }

    for visited_cell in state.visited_cells.iter() {
        let i = visited_cell.0;
        let j = visited_cell.1;

        input_vec[i][j] = '#';

<<<<<<< Updated upstream
        if is_in_loop(&input_vec, initial_i, initial_j, initial_direction) {
            total_steps += 1;
=======
 
    while i>=0 && i < input_vec.len() && j >= 0 && j < input_vec[i].len() {
        let next_coordinate = get_next_coordinate(i, j, direction);
        match next_coordinate {
            Some(next_coord) => {
                if next_coord.0 >= 0 && next_coord.0 < input_vec.len() && next_coord.1 >= 0 && next_coord.1 < input_vec[next_coord.0].len() {
                    if input_vec[next_coord.0][next_coord.1] == '#'{
                        direction = get_new_direction(direction);
                    }
                    else {
                        if !(next_coord.0 == initial_i && next_coord.1 == initial_j) {

                            input_vec[next_coord.0][next_coord.1] = '#';
                            
                            if is_in_loop(&input_vec, i, j, direction) {
                                total_steps += 1;
                            }

                            input_vec[next_coord.0][next_coord.1] = '.'; // Revert back the barrier
                        }
                        
                        i = next_coord.0;
                        j = next_coord.1;
                    }
                } else {
                    break;
                }
            },
            None => {
                // Handle the case when there's no valid next coordinate
                break;
            }
>>>>>>> Stashed changes
        }

        input_vec[i][j] = '.';
    }

    return total_steps.to_string(); 

}
