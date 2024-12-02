// Advent of Code 2024 - Day 1
// Problem link: https://adventofcode.com/2024/day/1

pub fn solve() {
    //Create a priority queue 1
     let exe_path = std::env::current_exe().unwrap();
     let exe_dir = exe_path.parent().unwrap();
     let input_file_path = exe_dir.join("../../src/days/day1/input.txt");
     print!("Input file path: {}\n", input_file_path.display());

    let input = match std::fs::read_to_string(input_file_path) {
        Ok(content) => content,
        Err(e) => panic!("Something went wrong reading the file: {}", e),
    };

    let mut pq1 = std::collections::BinaryHeap::new();
    //Create a priority queue 2
    let mut pq2 = std::collections::BinaryHeap::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace();
        let num1 = nums.next().unwrap().parse::<i32>().unwrap();
        let num2 = nums.next().unwrap().parse::<i32>().unwrap();
        pq1.push(std::cmp::Reverse(num1));
        pq2.push(std::cmp::Reverse(num2));
    }

    //Pop the top element from pq1 and pq2 and subtract them, take the absolute value and add it to a variable called sum_of_all
    let mut sum_of_all = 0;
    while !pq1.is_empty() {
        let num1 = pq1.pop().unwrap().0;
        let num2 = pq2.pop().unwrap().0;
        sum_of_all += (num1 - num2).abs();
    }

    print!("The sum of all the absolute differences is: {}", sum_of_all);
}
