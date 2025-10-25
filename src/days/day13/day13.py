# Advent of Code 2024 - Day 13
# Problem link: https://adventofcode.com/2024/day/13

import os
import numpy as np

def solve_part_one(input_data):
    """
    Solve part one of the problem.
    """
    # Implement Part One calculation logic here
    #
    #Button A: X+94, Y+34
    # Button B: X+22, Y+67
    # Prize: X=8400, Y=5400`
    parts = input_data.split('\n\n')
    total = 0
    
    for part in parts:
        sentences = part.split('\n')
        x_coefficients = [sentences[0].split(':')[1].strip().split(',')[0].strip().split('+')[1], sentences[0].split(':')[1].strip().split(',')[1].strip().split('+')[1]]
        y_coefficients = [sentences[1].split(':')[1].strip().split(',')[0].strip().split('+')[1], sentences[1].split(':')[1].strip().split(',')[1].strip().split('+')[1]]
        results = [sentences[2].split(':')[1].strip().split(',')[0].strip().split('=')[1], sentences[2].split(':')[1].strip().split(',')[1].strip().split('=')[1]]

        A = np.array([[int(x_coefficients[0]), int(y_coefficients[0])], [int(x_coefficients[1]), int(y_coefficients[1])]])
        b = np.array([int(results[0]), int(results[1])])

        solution = np.linalg.solve(A, b)

        #if each solution is a positive integer, we consider integer if it is very close to an integer
        if all(solution[i] >= 0 and abs(solution[i] - round(solution[i])) < 1e-5 for i in range(len(solution))):
            total = total+ int(round(solution[0]))*3 + int(round(solution[1]))*1

    print(total)
            
    return total

def solve_part_two(input_data):
    """
    Solve part two of the problem.
    """
    # Implement Part Two calculation logic here
    parts = input_data.split('\n\n')
    total = 0
    
    for part in parts:
        sentences = part.split('\n')
        x_coefficients = [sentences[0].split(':')[1].strip().split(',')[0].strip().split('+')[1], sentences[0].split(':')[1].strip().split(',')[1].strip().split('+')[1]]
        y_coefficients = [sentences[1].split(':')[1].strip().split(',')[0].strip().split('+')[1], sentences[1].split(':')[1].strip().split(',')[1].strip().split('+')[1]]
        results = [sentences[2].split(':')[1].strip().split(',')[0].strip().split('=')[1], sentences[2].split(':')[1].strip().split(',')[1].strip().split('=')[1]]

        A = np.array([[int(x_coefficients[0]), int(y_coefficients[0])], [int(x_coefficients[1]), int(y_coefficients[1])]])
        b = np.array([int(results[0])+1e13, int(results[1])+1e13])

        solution = np.linalg.solve(A, b)


        #if each solution is a positive integer, we consider integer if it is very close to an integer
        if all(solution[i] >= 0 and abs(solution[i] - round(solution[i])) < 1e-3 for i in range(len(solution))):

            total = total+ int(round(solution[0]))*3 + int(round(solution[1]))*1

    print(total)
            
    return total

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
    
    print(f"=== Advent of Code 2024 - Day 13 ===")
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
