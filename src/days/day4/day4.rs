// Advent of Code 2024 - Day 4
// Problem link: https://adventofcode.com/2024/day/4

pub fn solve() {
    println!("Solution for Day 4");
    let DAY = 4;
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

fn calculate_part_one_result(input: &str) -> String {
    // Implement Part One calculation logic here
    let input_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut occurance_count = 0;

    for i in 0..input_vec.len() {
        for j in 0..input_vec[i].len() {
            if input_vec[i][j] == 'X' {
                if j + 3 < input_vec[i].len() {
                    if input_vec[i][j + 1] == 'M' && input_vec[i][j + 2] == 'A' && input_vec[i][j + 3] == 'S' {
                        occurance_count += 1;
                    }
                }
                if i + 3 < input_vec.len() {
                    if input_vec[i + 1][j] == 'M' && input_vec[i + 2][j] == 'A' && input_vec[i + 3][j] == 'S' {
                        occurance_count += 1;
                    }
                }
                if i + 3 < input_vec.len() && j + 3 < input_vec[i].len() {
                    if input_vec[i + 1][j + 1] == 'M' && input_vec[i + 2][j + 2] == 'A' && input_vec[i + 3][j + 3] == 'S' {
                        occurance_count += 1;
                    }
                }
                if i + 3 < input_vec.len() && j >= 3 {
                    if input_vec[i + 1][j - 1] == 'M' && input_vec[i + 2][j - 2] == 'A' && input_vec[i + 3][j - 3] == 'S' {
                        occurance_count += 1;
                    }
                }
            }
            if input_vec[i][j] == 'S' {
                if j + 3 < input_vec[i].len() {
                    if input_vec[i][j + 1] == 'A' && input_vec[i][j + 2] == 'M' && input_vec[i][j + 3] == 'X' {
                        occurance_count += 1;
                    }
                }
                if i + 3 < input_vec.len() {
                    if input_vec[i + 1][j] == 'A' && input_vec[i + 2][j] == 'M' && input_vec[i + 3][j] == 'X' {
                        occurance_count += 1;
                    }
                }
                if i + 3 < input_vec.len() && j + 3 < input_vec[i].len() {
                    if input_vec[i + 1][j + 1] == 'A' && input_vec[i + 2][j + 2] == 'M' && input_vec[i + 3][j + 3] == 'X' {
                        occurance_count += 1;
                    }
                }
                if i + 3 < input_vec.len() && j >= 3 {
                    if input_vec[i + 1][j - 1] == 'A' && input_vec[i + 2][j - 2] == 'M' && input_vec[i + 3][j - 3] == 'X' {
                        occurance_count += 1;
                    }
                }
            }
        }
    }

    occurance_count.to_string()
}

fn calculate_part_two_result(input: &str) -> String {
    // Implement Part Two calculation logic here
    let input_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    
    //loop through the input_vec and check for the character "M", if M, then check if the right down diagonal characters are "A" and "S", and then also check if the left down diagonal from i+2 column is also "MAS" or "SAM"
    //if all conditions are met, then increment the count
    let mut occurance_count = 0;
    for i in 0..input_vec.len() {
        for j in 0..input_vec[i].len() {
            if input_vec[i][j] == 'M' || input_vec[i][j] == 'S' {
                if j+2 < input_vec[i].len() && i+2 < input_vec.len() {
                    if input_vec[i][j] == 'M' {
                        if input_vec[i+1][j+1] == 'A' && input_vec[i+2][j+2] == 'S' {
                            //Check left down diagonal starting from j+2 in the ith row, the characters should be "MAS" or "SAM"
                            if(j+2 < input_vec[i].len() && i+2 < input_vec.len()) {
                                if((input_vec[i][j+2] == 'M' && input_vec[i+1][j+1] == 'A' && input_vec[i+2][j] == 'S') || (input_vec[i][j+2] == 'S' && input_vec[i+1][j+1] == 'A' && input_vec[i+2][j] == 'M')) {
                                    occurance_count += 1;
                                }
                            }
                        }
                    }

                    if input_vec[i][j] == 'S' {
                        if input_vec[i+1][j+1] == 'A' && input_vec[i+2][j+2] == 'M' {
                            //Check left down diagonal starting from j+2 in the ith row, the characters should be "MAS" or "SAM"
                            if(j+2 < input_vec[i].len() && i+2 < input_vec.len()) {
                                if((input_vec[i][j+2] == 'M' && input_vec[i+1][j+1] == 'A' && input_vec[i+2][j] == 'S') || (input_vec[i][j+2] == 'S' && input_vec[i+1][j+1] == 'A' && input_vec[i+2][j] == 'M')) {
                                    occurance_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return occurance_count.to_string();
}








