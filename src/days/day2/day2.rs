// Advent of Code 2024 - Day 2
// Problem link: https://adventofcode.com/2024/day/2

use std::array;

pub fn solve() {
    println!("Solution for Day 2");
    let DAY = 2;
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    let input_file_path = exe_dir.join(format!("../../src/days/day{}/input.txt", DAY));
    println!("Input file path: {}", input_file_path.display());

    let input_content = match std::fs::read_to_string(input_file_path) {
        Ok(content) => content,
        Err(e) => panic!("Something went wrong reading the input file: {}", e),
    };

    let test_input_file_path = exe_dir.join(format!("../../src/days/day{}/test_input.txt", DAY));
    println!("Test input file path: {}", test_input_file_path.display());

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

fn is_report_valid(report_arr: &Vec<i32>) -> bool {
    let isIncreasing = report_arr.windows(2).all(|w| w[0] < w[1] && (w[0]-w[1]).abs() >= 1 && (w[0]-w[1]).abs() <= 3);
    let isDecreasing = report_arr.windows(2).all(|w| w[0] > w[1] && (w[0]-w[1]).abs() >= 1 && (w[0]-w[1]).abs() <= 3);

    return isIncreasing || isDecreasing;
}

fn is_report_valid_part_two(report_array: Vec<i32>) -> bool {
    let initial_vec = report_array.clone();
    if is_report_valid(&initial_vec) {
        return true;
    }
    for i in 0..report_array.len() {
        let mut new_vec = report_array.clone();
        new_vec.remove(i);
        if is_report_valid(&new_vec) {
            return true;
        }
    }

    return false;

}


fn calculate_part_one_result(input: &str) -> String {
    // Implement Part One calculation logic here
    let mut count = 0;
    
    for line in input.lines() {
        let vec: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        
        if is_report_valid(&vec) {
            count += 1;
        }
    }

    return count.to_string();
}

fn calculate_part_two_result(input: &str) -> String {
    // Implement Part Two calculation logic here
    let mut count = 0;

    for line in input.lines() {
        let vec: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        
        if is_report_valid_part_two(vec) {
            count += 1;
        }
    }

    return count.to_string();
}

