// Advent of Code 2024 - Day 6
// Problem link: https://adventofcode.com/2024/day/6

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

    // Call solvePartOne with both inputs
    solvePartOne(&input_content, &test_input_content);

    // Call solvePartTwo with both inputs
    solvePartTwo(&input_content, &test_input_content);
}

fn solvePartOne(input: &str, test_input: &str) {
    println!("Part One:");
    println!("Test input result: {}", calculate_part_one_result(test_input));
    println!("Input result: {}", calculate_part_one_result(input));
}

fn solvePartTwo(input: &str, test_input: &str) {
    println!("Part Two:");
    println!("Test input result: {}", calculate_part_two_result(test_input));
    println!("Input result: {}", calculate_part_two_result(input));
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

fn calculate_part_one_result(input: &str) -> String {
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

fn is_in_loop(input_vec: &Vec<Vec<char>>, i: usize, j: usize, current_direction: char) -> bool {
    let mut visited_cells: std::collections::HashSet<(usize, usize, char)> = std::collections::HashSet::new();
    let mut direction = current_direction;
    let mut new_i = i;
    let mut new_j = j;

    loop {
        if visited_cells.contains(&(new_i, new_j, direction)) {
            return true;
        }
        
        visited_cells.insert((new_i, new_j, direction));
        
        match get_next_coordinate(new_i, new_j, direction) {
            Some(next_coord) => {
                if next_coord.0 >= input_vec.len() || next_coord.1 >= input_vec[next_coord.0].len() {
                    break;
                }
                
                if input_vec[next_coord.0][next_coord.1] == '#'{
                    direction = get_new_direction(direction);
                } else {
                    new_i = next_coord.0;
                    new_j = next_coord.1;
                }
            },
            None => break,
        }
    }
    
    false
}

fn calculate_part_two_result(input: &str) -> String {
    // Implement Part Two calculation logic here
    let mut input_vec: Vec<Vec<char>> = input.lines().map(|line | line.chars().collect()).collect();

    let mut initial_i = 0;
    let mut initial_j = 0;

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
    let mut direction = input_vec[i][j];

    // Follow the path, at each step, first check if a barrier was placed in the next cell in the same direction, then after that, whatever the new direction would be, in that if in the same column or row (based on the new direction), if there is another barrier, then check if a loop will be created or not, if yes, place a barrier "#" and increment a counter
    let mut total_steps = 0;

    // Keep track of discovered loops
    let mut discovered_loops: std::collections::HashSet<(usize, usize, char)> = std::collections::HashSet::new();

 
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
                                //print the grid
                                for i in 0..input_vec.len() {
                                    for j in 0..input_vec[i].len() {
                                        print!("{}", input_vec[i][j]);
                                    }
                                    println!();
                                }
                                println!();
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
        }
    }

    return total_steps.to_string();
}


// fn calculate_part_two_result(input: &str) -> String {
//     // Implement Part Two calculation logic here
//     let mut input_vec: Vec<Vec<char>> = input.lines().map(|line | line.chars().collect()).collect();

//     let mut initial_i = 0;
//     let mut initial_j = 0;

//     //Find the initial position of the player
//     for i in 0..input_vec.len() {
//         for j in 0..input_vec[i].len() {
//             if input_vec[i][j] == '^' || input_vec[i][j] == 'v' || input_vec[i][j] == '<' || input_vec[i][j] == '>' {
//                 initial_j = j;
//                 initial_i = i;
//             }
//         }
//     }

//     let mut i = initial_i;
//     let mut j = initial_j;
//     let mut direction = input_vec[i][j];

//     // Follow the path, at each step, first check if a barrier was placed in the next cell in the same direction, then after that, whatever the new direction would be, in that if in the same column or row (based on the new direction), if there is another barrier, then check if a loop will be created or not, if yes, place a barrier "#" and increment a counter
//     let mut total_steps = 0;

//     // Keep track of discovered loops
//     let mut discovered_loops: std::collections::HashSet<(usize, usize, char)> = std::collections::HashSet::new();

 
//     while i>=0 && i < input_vec.len() && j >= 0 && j < input_vec[i].len() {
//         let next_coordinate = get_next_coordinate(i, j, direction);
//         match next_coordinate {
//             Some(next_coord) => {
//                 if next_coord.0 >= 0 && next_coord.0 < input_vec.len() && next_coord.1 >= 0 && next_coord.1 < input_vec[next_coord.0].len() {
//                     if input_vec[next_coord.0][next_coord.1] == '#'{
//                         direction = get_new_direction(direction);
//                     }
//                     else {               
//                         if !(next_coord.0 == initial_i && next_coord.1 == initial_j) {
                     
                            
//                             input_vec[next_coord.0][next_coord.1] = '#';
             
//                                 if is_in_loop(&input_vec, i, j, direction) {
//                                     total_steps += 1;
            
//                                 }
                   
//                             input_vec[next_coord.0][next_coord.1] = '.'; // Revert back the barrier
//                         }
                        
//                         i = next_coord.0;
//                         j = next_coord.1;
//                     }
//                 } else {
//                     break;
//                 }
//             },
//             None => {
//                 // Handle the case when there's no valid next coordinate
//                 break;
//             }
//         }
//     }

//     return total_steps.to_string();
// }

