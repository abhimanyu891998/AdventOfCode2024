#!/bin/bash

# Check if day number is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <day_number>"
  exit 1
fi

DAY=$1
FILENAME="./src/days/day${DAY}.rs"
PROBLEM_LINK="https://adventofcode.com/2024/day/${DAY}"

# Create the day's Rust file with the problem link comment
cat <<EOL > $FILENAME
// Advent of Code 2025 - Day ${DAY}
// Problem link: ${PROBLEM_LINK}

pub fn solve() {
    println!("Solution for Day ${DAY}");
    // Your solution code here
}
EOL

# Update the mod.rs file to include the new module
echo "pub mod day${DAY};" >> src/days/mod.rs

# Update the main.rs file to call the new day's solve function
sed -i '' "/fn main() {/a\\
    days::day${DAY}::solve();
" src/main.rs

echo "Generated $FILENAME with problem link $PROBLEM_LINK"