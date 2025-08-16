# Advent of Code 2024 - Day 10
# Problem link: https://adventofcode.com/2024/day/10

import os


def is_valid_coordinate(hill_matrix, row, col):
    rows_count = len(hill_matrix)
    cols_count = len(hill_matrix[0])
    
    if row < 0 or row >= rows_count or col < 0 or col >= cols_count:
        return False
    return True


def dfs(hill_matrix, row, col, prev_value, visited):
    if not is_valid_coordinate(hill_matrix, row, col):
        return 0

    if hill_matrix[row][col] != prev_value + 1:
        return 0

    if hill_matrix[row][col] == 9:
        if (row, col) not in visited:
            visited.add((row, col))
            return 1
        else:
            return 0
    
    return dfs(hill_matrix, row + 1, col, hill_matrix[row][col], visited) + dfs(hill_matrix, row - 1, col, hill_matrix[row][col], visited) + dfs(hill_matrix, row, col + 1, hill_matrix[row][col], visited) + dfs(hill_matrix, row, col - 1, hill_matrix[row][col], visited)



def solve_part_one(input_data):
    """
    Solve part one of the problem.
    """
    hill_matrix = []

    for line in input_data.split("\n"):
        hill_matrix.append([int(char) for char in line])

    total_count = 0
    visited = set()
    for row in range(len(hill_matrix)):
        for col in range(len(hill_matrix[row])):
            visited.clear()
            if hill_matrix[row][col] == 0:
                count = dfs(hill_matrix, row, col, -1, visited)
                total_count += count
                

    return total_count


def dfs_part_two(hill_matrix, row, col, prev_value):
    if not is_valid_coordinate(hill_matrix, row, col):
        return 0

    if hill_matrix[row][col] != prev_value + 1:
        return 0
    
    
    if hill_matrix[row][col] == 9:
        return 1
    
    return dfs_part_two(hill_matrix, row + 1, col, hill_matrix[row][col]) + dfs_part_two(hill_matrix, row - 1, col, hill_matrix[row][col]) + dfs_part_two(hill_matrix, row, col + 1, hill_matrix[row][col]) + dfs_part_two(hill_matrix, row, col - 1, hill_matrix[row][col])


def solve_part_two(input_data):
    """
    Solve part two of the problem.
    """
    # Implement Part Two calculation logic here
    hill_matrix = []

    for line in input_data.split("\n"):
        hill_matrix.append([int(char) for char in line])

    total_count = 0
    for row in range(len(hill_matrix)):
        for col in range(len(hill_matrix[row])):
            if hill_matrix[row][col] == 0:
                count = dfs_part_two(hill_matrix, row, col, -1)
                total_count += count
                

    return total_count

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
    
    print(f"=== Advent of Code 2024 - Day 10 ===")
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
