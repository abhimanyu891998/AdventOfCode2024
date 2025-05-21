#!/bin/bash

# Check if day number is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <day_number>"
  exit 1
fi

DAY=$1
DIR="./src/days/day${DAY}"
FILENAME="${DIR}/day${DAY}.rs"
INPUT_FILE="${DIR}/input.txt"
TEST_INPUT_FILE="${DIR}/test_input.txt"
MOD_FILE="${DIR}/mod.rs"
PROBLEM_LINK="https://adventofcode.com/2024/day/${DAY}"

# Create a directory for the day's files
mkdir -p "$DIR"

# Create the day's Rust file with the problem link comment
cat <<EOL > "$FILENAME"
// Advent of Code 2024 - Day ${DAY}
// Problem link: ${PROBLEM_LINK}

pub fn solve() {
    println!("Solution for Day ${DAY}");
    let day = ${DAY};
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

    // Call solve_part_one with both inputs
    solve_part_one(&input_content, &test_input_content);

    // Call solve_part_two with both inputs
    solve_part_two(&input_content, &test_input_content);
}

fn solve_part_one(input: &str, test_input: &str) {
    println!("Part One:");
    println!("Test input result: {}", calculate_part_one_result(test_input));
    println!("Input result: {}", calculate_part_one_result(input));
}

fn solve_part_two(input: &str, test_input: &str) {
    println!("Part Two:");
    println!("Test input result: {}", calculate_part_two_result(test_input));
    println!("Input result: {}", calculate_part_two_result(input));
}

fn calculate_part_one_result(input: &str) -> String {
    // Implement Part One calculation logic here
    return "Not implemented yet".to_string();
}

fn calculate_part_two_result(input: &str) -> String {
    // Implement Part Two calculation logic here
    return "Not implemented yet".to_string();
}

EOL

#Create an empty input.txt file
touch "$INPUT_FILE"
touch "$TEST_INPUT_FILE"

# Create the day's mod.rs file
cat <<EOM > "$MOD_FILE"
// src/days/day${DAY}/mod.rs
pub mod day${DAY};

// Re-export everything from day${DAY}
pub use self::day${DAY}::*;
EOM




# Update the mod.rs file to include the new module
echo "pub mod day${DAY};" >> src/days/mod.rs



# Update the main.rs file to call the new day's solve function
sed -i '' "/fn main() {/a\\
    days::day${DAY}::solve();
" src/main.rs

echo "Generated $FILENAME with problem link $PROBLEM_LINK and input file $INPUT_FILE"