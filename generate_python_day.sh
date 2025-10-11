#!/bin/bash

# Check if day number is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <day_number>"
  exit 1
fi

DAY=$1
DIR="./src/days/day${DAY}"
FILENAME="${DIR}/day${DAY}.py"
INPUT_FILE="${DIR}/input.txt"
TEST_INPUT_FILE="${DIR}/test_input.txt"
PROBLEM_LINK="https://adventofcode.com/2024/day/${DAY}"

# Create a directory for the day's files
mkdir -p "$DIR"

# Create the day's Python file with the problem link comment
cat <<EOL > "$FILENAME"
# Advent of Code 2024 - Day ${DAY}
# Problem link: ${PROBLEM_LINK}

import os

def solve_part_one(input_data):
    """
    Solve part one of the problem.
    """
    # Implement Part One calculation logic here
    return "Not implemented yet"

def solve_part_two(input_data):
    """
    Solve part two of the problem.
    """
    # Implement Part Two calculation logic here
    return "Not implemented yet"

def load_input(filename):
    """
    Load input from file.
    """
    script_dir = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(script_dir, filename)
    with open(file_path, 'r') as f:
        return f.read().strip()

def main():
    # Load test input and actual input
    test_input = load_input('test_input.txt')
    actual_input = load_input('input.txt')
    
    print(f"=== Advent of Code 2024 - Day ${DAY} ===")
    print()
    
    # Part One
    print("Part One:")
    print(f"Test input result: {solve_part_one(test_input)}")
    print(f"Actual input result: {solve_part_one(actual_input)}")
    print()
    
    # Part Two
    print("Part Two:")
    print(f"Test input result: {solve_part_two(test_input)}")
    print(f"Actual input result: {solve_part_two(actual_input)}")

if __name__ == "__main__":
    main()
EOL

# Create empty input files
touch "$INPUT_FILE"
touch "$TEST_INPUT_FILE"

echo "Generated $FILENAME with problem link $PROBLEM_LINK"
echo "Created input files: $INPUT_FILE and $TEST_INPUT_FILE"
echo "Run with: cd src/days/day${DAY} && python day${DAY}.py"