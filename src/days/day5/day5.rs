// Advent of Code 2024 - Day 5
// Problem link: https://adventofcode.com/2024/day/5

pub fn solve() {
    println!("Solution for Day 5");
    let DAY = 5;
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
    let mut priority_dict = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
    let mut is_priority_input_read = false;
    let mut median_sum_of_valid_prints = 0;

    for line in input.lines() {
        if line == "" {
            is_priority_input_read = true;
            continue;
        }

        if is_priority_input_read == false {
            let mut parts = line.split("|");
            let priority_before = parts.next().unwrap().parse::<i32>().unwrap();
            let value = parts.next().unwrap().parse::<i32>().unwrap();
            let mut set = priority_dict.entry(value).or_insert(std::collections::HashSet::new());
            set.insert(priority_before);
        }
        else {
            // Implement the logic to read the input after the empty line
            // Line will be comma separated integers, we can store then in a vector
            let mut parts = line.split(",");
            let mut input_vec = Vec::<i32>::new();
            for part in parts {
                input_vec.push(part.parse::<i32>().unwrap());
            }

            let mut is_valid = true;

            for i in 0..input_vec.len()-1 {
                let mut d = i+1;
                let slice = &input_vec[d..];
                //Check if the slice has any intersecting values with the hasshet of the current value
                let curr_value = input_vec.get(i).unwrap();
                //check if the hashset for the curr_value as key in the priority dict has any interseting values with the slice, if yes then the slice is invalid
                if let Some(set) = priority_dict.get(&curr_value) {
                    for value in slice {
                        if set.contains(value) {
                            is_valid = false;
                            break
                        }
                    }
                }    
            }

            if is_valid {
                //Add the middle value of the given input_vec to the sum
                let middle_index = input_vec.len() / 2;
                let middle_value = input_vec.get(middle_index).unwrap();
                median_sum_of_valid_prints += middle_value;
            }
        }
    }


    return median_sum_of_valid_prints.to_string();
}

fn calculate_part_two_result(input: &str) -> String {
    let mut priority_dict = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
    let mut is_priority_input_read = false;
    let mut median_sum_of_valid_prints = 0;

    for line in input.lines() {
        if line == "" {
            is_priority_input_read = true;
            continue;
        }

        if is_priority_input_read == false {
            let mut parts = line.split("|");
            let priority_before = parts.next().unwrap().parse::<i32>().unwrap();
            let value = parts.next().unwrap().parse::<i32>().unwrap();
            let mut set = priority_dict.entry(value).or_insert(std::collections::HashSet::new());
            set.insert(priority_before);
        }
        else {
            let mut parts = line.split(",");
            let mut input_vec = Vec::<i32>::new();
            for part in parts {
                input_vec.push(part.parse::<i32>().unwrap());
            }

            let mut is_valid = true;
            let len = input_vec.len();
            let mut i = 0;

            while i < len - 1 {
                let mut swapped = false;
                if let Some(set) = priority_dict.get(&input_vec[i]) {
                    for j in (i+1)..len {
                        if set.contains(&input_vec[j]) {
                            is_valid = false;
                            input_vec.swap(i, j);
                            swapped = true;
                            break;  // Break after swap to recheck the same index
                        }
                    }
                }
                if !swapped {
                    i += 1;  // Only increment if no swap occurred
                }
            }

            if is_valid == false {
                // println!("Swapped input_vec: {:?}", input_vec);
                let middle_index = input_vec.len() / 2;
                let middle_value = input_vec[middle_index];
                median_sum_of_valid_prints += middle_value;
            }
        }
    }

    return median_sum_of_valid_prints.to_string();
}


