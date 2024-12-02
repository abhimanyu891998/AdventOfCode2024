// Advent of Code 2024 - Day 1
// Problem link: https://adventofcode.com/2024/day/1

pub fn solve() {
    println!("Solution for Day 1");
    let DAY = 1;
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
    return String::from(sum_of_all.to_string());
}

fn calculate_part_two_result(input: &str) -> String {
    let mut dict2 = std::collections::HashMap::new();
    let mut list1 = Vec::new();
    for line in input.lines() {
        let mut nums = line.split_whitespace();
        let num1 = nums.next().unwrap().parse::<i32>().unwrap();
        let num2 = nums.next().unwrap().parse::<i32>().unwrap();
        //increment the occurance of num2 in dict2
        *dict2.entry(num2).or_insert(0) += 1;
        //add num1 to hashset1
        list1.push(num1);
    }

    let mut sum_of_all = 0;

    //Iterate over the hashset1
    for num in list1.iter(){
        //if the num is in dict2, multiple the num with its number of occurances
        if dict2.contains_key(num){
            let occurances = dict2.get(num).unwrap();
            sum_of_all += num * dict2.get(num).unwrap();
        }
    }

    return String::from(sum_of_all.to_string());
}

