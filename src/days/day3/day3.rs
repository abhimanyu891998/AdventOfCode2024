// Advent of Code 2024 - Day 3
// Problem link: https://adventofcode.com/2024/day/3

use regex::Regex;

pub fn solve() {
    println!("Solution for Day 3");
    let DAY = 3;
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
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum_of_all = 0;
    for cap in re.captures_iter(input) {
        sum_of_all += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
    }
    
    return sum_of_all.to_string();
}

fn calculate_part_two_result(input: &str) -> String {
    // Implement Part Two calculation logic here
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    //Create a regex for the do() and dont() functions
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    // Create a stack, while looping the input, if we find a mul_regex substring, and there is not a don't() function at the top of the stack, we push the mul() result to the stack, if there is a do(), push it to the stack, if there is a don't(), push that to the stack as well
    let mut stack: Vec<String> = Vec::new();
    
    let mut i = 0;
    // while i < input.len() {
    //     if let Some(cap) = mul_regex.captures(&input[i..]) {
    //         let mul_result = cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
    //         if stack.last() != Some(&"don't()".to_string()) {
    //             stack.push(mul_result.to_string());
    //         }
    //         i += cap.get(0).unwrap().end();
    //     } else if let Some(cap) = do_regex.captures(&input[i..]) {
    //         print!("do() ");
    //         stack.push("do()".to_string());
    //         i += cap.get(0).unwrap().end();
    //     } else if let Some(cap) = dont_regex.captures(&input[i..]) {
    //         print!("don't() ");
    //         stack.push("don't()".to_string());
    //         i += cap.get(0).unwrap().end();
    //     } else {
    //         i += 1;
    //     }
    // }

    let mut sum_of_all = 0;
    while i< input.len() {
        // Check if the current character is a 'm' and then check if the next characters are 'ul(', if yes, then check till the next ')' and confirm if the mul_regex is matched and push the result to the stack
        if input[i..].starts_with("mul(") {
            let mut j = i + 4;
            while j < input.len() && input[j..].starts_with(")") == false {
                j += 1;
            }
            if let Some(cap) = mul_regex.captures(&input[i..j+1]) {
                let mul_result = cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
                if stack.last() != Some(&"don't()".to_string()) {
                    stack.push(mul_result.to_string());
                    sum_of_all += mul_result;
                }
            }
            i = j + 1;
        } else if input[i..].starts_with("do()") {
            stack.push("do()".to_string());
            i += 4;
        } else if input[i..].starts_with("don't()") {
            stack.push("don't()".to_string());
            i += 7;
        } else {
            i += 1;
        }
    }

    return sum_of_all.to_string();
}

