// Advent of Code 2024 - Day 9
// Problem link: https://adventofcode.com/2024/day/9

use std::collections::HashMap;

pub fn solve() {
    println!("Solution for Day 9");
    let day = 9;
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    let input_file_path = exe_dir.join(format!("../../src/days/day{}/input.txt", day));

    let input_content = match std::fs::read_to_string(input_file_path) {
        Ok(content) => content,
        Err(e) => panic!("Something went wrong reading the input file: {}", e),
    };

    let test_input_file_path = exe_dir.join(format!("../../src/days/day{}/test_input.txt", day));

    let test_input_content = match std::fs::read_to_string(test_input_file_path) {
        Ok(content) => content,
        Err(e) => panic!("Something went wrong reading the test input file: {}", e),
    };

    // Call solvePartOne with both inputs
    solve_part_one(&input_content, &test_input_content);
    // Call solvePartTwo with both inputs
    solve_part_two(&input_content, &test_input_content);
}

fn fill_disc_space(result: &mut Vec<i32>, space_idx: i32, space_length: i32) {
    if (space_length == 0) {
        return;
    }

    for i in 0..space_length {
        result.push(space_idx);
    }
}

fn fill_empty_space(result: &mut Vec<i32>, empty_space_length: i32) {
    for i in 0..empty_space_length {
        result.push(-1);
    }
}

fn create_integer_array(input: &str) -> Vec<i32> {
    let mut result = Vec::new();

    //enumerate the input string
    for (idx, char) in input.chars().enumerate() {
        if (idx % 2 == 0) {
            fill_disc_space(
                &mut result,
                (idx / 2).try_into().unwrap(),
                char.to_digit(10).unwrap() as i32,
            );
        } else {
            fill_empty_space(&mut result, char.to_digit(10).unwrap() as i32);
        }
    }
    return result;
}

fn find_next_empty_space(integer_array: &mut Vec<i32>, prev_empty_space_idx: i32) -> i32 {
    let mut prev_empty_space_idx = prev_empty_space_idx.clone();

    if prev_empty_space_idx == -1 {
        prev_empty_space_idx = 0;
    }

    if let Ok(prev_empty_space_idx_usize) = usize::try_from(prev_empty_space_idx) {
        for i in prev_empty_space_idx_usize..integer_array.len() {
            if integer_array[i] == -1 {
                return i.try_into().unwrap();
            }
        }
    }

    return -1;
}

fn find_first_free_space(integer_array: &mut Vec<i32>) -> i32 {
    for i in 0..integer_array.len() {
        if integer_array[i] == -1 {
            return i as i32;
        }
    }

    return -1;
}

fn defrag_disk(integer_array: &mut Vec<i32>) {
    let mut curr_empty_space_idx: i32 = find_first_free_space(integer_array);
    let mut curr_moving_idx = (integer_array.len() - 1) as i32;

    while curr_empty_space_idx <= curr_moving_idx {
        if (curr_empty_space_idx == -1) {
            break;
        }

        //ignore free space
        while (integer_array[curr_moving_idx as usize] == -1) {
            curr_moving_idx -= 1;
            continue;
        }

        if (curr_empty_space_idx == -1) {
            break;
        }

        // move the current moving idx to the empty space
        integer_array[curr_empty_space_idx as usize] = integer_array[curr_moving_idx as usize];
        integer_array[curr_moving_idx as usize] = -1;

        // decrement the current moving idx
        curr_moving_idx -= 1;
        // find the next empty space
        curr_empty_space_idx = find_next_empty_space(integer_array, curr_empty_space_idx);
    }
}

fn calculate_checksum(integer_array: &mut Vec<i32>) -> u64 {
    let mut checksum: u64 = 0;

    for i in 0..integer_array.len() {
        if integer_array[i] == -1 {
            break;
        }

        checksum += (i as u64) * integer_array[i] as u64;
    }

    return checksum;
}

struct FileSpace {
    start_idx: i32,
    size: i32,
}

struct FreeSpace {
    start_idx: i32,
    size: i32,
}

fn calculate_part_one_result(input: &str) -> String {
    // Implement Part One calculation logic here
    let mut integer_array = create_integer_array(input);
    defrag_disk(&mut integer_array);
    let checksum = calculate_checksum(&mut integer_array);
    return checksum.to_string();
}

fn calculate_part_two_result(input: &str) -> String {
    let mut file_space_map: HashMap<i32, FileSpace> = HashMap::new();
    let mut blank_spaces = Vec::new();

    let mut next_start_idx = 0;
    for (idx, char) in input.chars().enumerate() {
        let size = char.to_digit(10).unwrap() as i32;

        if idx % 2 == 0 {
            if size == 0 {
                println!("Size is 0 at index {}", idx);
                panic!("Size is 0");
            }
            let file_idx = (idx / 2) as i32;
            file_space_map.insert(
                file_idx,
                FileSpace {
                    start_idx: next_start_idx as i32,
                    size: char.to_digit(10).unwrap() as i32,
                },
            );
        } else {
            // Empty else block - removed the "em" which was causing an error
            if size == 0 {
                continue;
            }
            blank_spaces.push(FreeSpace {
                start_idx: next_start_idx as i32,
                size: char.to_digit(10).unwrap() as i32,
            });
        }

        next_start_idx = next_start_idx + size;
    }

    //Iterate through file space in the reverse order of key value
    let mut keys: Vec<_> = file_space_map.keys().cloned().collect();
    keys.sort_by(|a, b| b.cmp(a)); // Sort in reverse order

    for key in &keys {
        let value = &mut file_space_map.get_mut(&key).unwrap();
        let start_idx = value.start_idx;
        let size = value.size;

        for (idx, free_space) in blank_spaces.iter_mut().enumerate() {
            if free_space.start_idx < start_idx {
                if free_space.size == size {
                    value.start_idx = free_space.start_idx;
                    //Remove this free space from the blank spaces
                    blank_spaces.remove(idx);
                    break;
                }
                if free_space.size > size {
                    value.start_idx = free_space.start_idx;
                    free_space.start_idx = free_space.start_idx + size;
                    free_space.size = free_space.size - size;
                    break;
                }
            } else {
                break;
            }
        }
    }

    //Calculate the checksum
    let mut checksum: i64 = 0;
    for key in &keys {
        let value = &file_space_map[&key];
        let mut start_idx = value.start_idx;

        while start_idx < value.start_idx + value.size {
            checksum += (key * start_idx) as i64;
            start_idx += 1;
        }
    }
    return checksum.to_string();
}

fn solve_part_one(input: &str, test_input: &str) {
    println!("Part One:");
    println!(
        "Test input result: {}",
        calculate_part_one_result(test_input)
    );
    println!("Input result: {}", calculate_part_one_result(input));
}

fn solve_part_two(input: &str, test_input: &str) {
    println!("Part Two:");
    println!(
        "Test input result: {}",
        calculate_part_two_result(test_input)
    );
    println!("Input result: {}", calculate_part_two_result(input));
}
