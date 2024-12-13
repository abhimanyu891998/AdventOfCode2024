// Advent of Code 2024 - Day 7
// Problem link: https://adventofcode.com/2024/day/7

pub fn solve() {
    println!("Solution for Day 7");
    let DAY = 7;
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

fn is_valid_result(result:u64, curr_value:u64, nums: &Vec<u64>, index:usize) -> bool {

    if index == nums.len() {
        return result == curr_value;
    }

    return is_valid_result(result, curr_value + nums[index], nums, index + 1) || is_valid_result(result, curr_value * nums[index], nums, index + 1);
}


fn calculate_part_one_result(input: &str) -> String {

    let mut total_valid = 0;

    // each line looks like: 1929: 19 21 45 
    for line in input.lines() {
        let parts = line.split(":");
        let mut parts_iter = parts.into_iter();
        let target_num = parts_iter.next().unwrap().parse::<u64>().unwrap();
        let nums_vec = parts_iter.next().unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        
        if is_valid_result(target_num, nums_vec[0], &nums_vec, 1) {
            total_valid += target_num;
        }
        
    }
    return total_valid.to_string();
}

fn is_valid_result_part_two_allows_concats(result:u64, curr_value:u64, nums: &Vec<u64>, index:usize) -> bool {

    if index == nums.len() {
        return result == curr_value;
    }

    let concatenated_value = format!("{}{}", curr_value, nums[index]).parse::<u64>().unwrap();
    return is_valid_result_part_two_allows_concats(result, curr_value + nums[index], nums, index + 1) 
        || is_valid_result_part_two_allows_concats(result, curr_value * nums[index], nums, index + 1) 
        || is_valid_result_part_two_allows_concats(result, concatenated_value, nums, index + 1);
}

fn calculate_part_two_result(input: &str) -> String {
    let mut total_valid = 0;
    for line in input.lines() {
        let parts = line.split(":");
        let mut parts_iter = parts.into_iter();
        let target_num = parts_iter.next().unwrap().parse::<u64>().unwrap();
        let nums_vec = parts_iter.next().unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        if is_valid_result(target_num, nums_vec[0], &nums_vec, 1) {
            total_valid += target_num;
        }
        else if is_valid_result_part_two_allows_concats(target_num, nums_vec[0], &nums_vec, 1) {
            total_valid += target_num;
        }
    }
    return total_valid.to_string();
}

