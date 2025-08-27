# Advent of Code 2024 - Day 11
# Problem link: https://adventofcode.com/2024/day/11

import os

def split_if_has_even_digits(num):
    num_str = str(num)

    if len(num_str) % 2 == 0:
        #if trailing 0s in the second half, then only return first half and one 0 for the second half, otherwise return both halves
        if num_str[-len(num_str)//2:].endswith("0"):
            return num_str[:len(num_str)//2], "0"
        else:
            return num_str[:len(num_str)//2], num_str[len(num_str)//2:]
    else:
        "",""


memory = {}

def trim_zeros(num_str):
    trimmed = num_str.lstrip("0")
    if trimmed == "":
        return "0"
    return trimmed

def solve_stones(stones, blinks):
    if blinks == 0:
        return 1
    elif (stones, blinks) in memory:
        return memory[(stones, blinks)]
    elif stones == 0:
        val =  solve_stones(1, blinks - 1)
    elif len(str(stones)) % 2 == 0:
        num_str = str(stones)
        val = solve_stones(int(trim_zeros(num_str[:len(num_str)//2])), blinks - 1) + solve_stones(int(trim_zeros(num_str[len(num_str)//2:])), blinks - 1)
    else:
        val =  solve_stones(stones*2024, blinks - 1)

    memory[(stones, blinks)] = val
    return val



    

def solve_part_one(input_data):
    """
    Solve part one of the problem.
    """
    nums = [int(num) for num in input_data.split(" ")]
    
    return sum(solve_stones(num, 25) for num in nums)

def solve_part_two(input_data):
    """
    Solve part two of the problem.
    """
    nums = [int(num) for num in input_data.split(" ")]
    
    return sum(solve_stones(num, 75) for num in nums)

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
    
    print(f"=== Advent of Code 2024 - Day 11 ===")
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
