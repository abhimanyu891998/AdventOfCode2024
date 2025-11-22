# Advent of Code 2024 - Day 14
# Problem link: https://adventofcode.com/2024/day/14

import os

def solve_part_one(input_data):
    """
    Solve part one of the problem.
    """
    # Implement Part One calculation logic here
    lines = input_data.splitlines()
    #data structure to store two tuples
    #write the data structure below
    positions_velocities = []
    for line in lines:
        parts = line.split(" ")
        coordinates = parts[0].strip().split("=")[1].strip().split(",")
        velocity = parts[1].strip().split("=")[1].strip().split(",")
        positions_velocities.append([(int(coordinates[0]), int(coordinates[1])), (int(velocity[0]), int(velocity[1]))])

    #final positions after 100 time steps when the matrix dimensions
    final_positions = []
    dimension_x = 101
    dimension_y = 103

    for pos_vel in positions_velocities:
        final_x = (pos_vel[0][0] + pos_vel[1][0] * 100) % dimension_x
        final_y = (pos_vel[0][1] + pos_vel[1][1] * 100) % dimension_y
        
        # Adjust for negative values
        if final_x < 0:
            final_x += dimension_x
        if final_y < 0:
            final_y += dimension_y
        
        final_positions.append((final_x, final_y))

    
    #draw the matrix with final positions with count of points in each cell
    # matrix = [[0 for _ in range(dimension_x)] for _ in range(dimension_y)]
    # for pos in final_positions:
    #     matrix[pos[1]][pos[0]] += 1
    # for row in matrix:
    #     print(" ".join(str(cell) for cell in row))


    # for each quadrant, count the number of points
    quadrant_counts = [0, 0, 0, 0]

    for pos in final_positions:
        if pos[0] < dimension_x // 2 and pos[1] < dimension_y // 2:
            quadrant_counts[0] += 1
        elif pos[0] > dimension_x // 2 and pos[1] < dimension_y // 2:
            quadrant_counts[1] += 1
        elif pos[0] < dimension_x // 2 and pos[1] > dimension_y // 2:
            quadrant_counts[2] += 1
        elif pos[0] > dimension_x // 2 and pos [1] > dimension_y // 2:
            quadrant_counts[3] += 1

    multiplied_amount = 1
    for count in quadrant_counts:
        multiplied_amount *= count

    return multiplied_amount


def solve_part_two(input_data):
    """
    Solve part two of the problem.
    """
    lines = input_data.splitlines()
    #data structure to store two tuples
    #write the data structure below
    positions_velocities = []
    for line in lines:
        parts = line.split(" ")
        coordinates = parts[0].strip().split("=")[1].strip().split(",")
        velocity = parts[1].strip().split("=")[1].strip().split(",")
        positions_velocities.append([(int(coordinates[0]), int(coordinates[1])), (int(velocity[0]), int(velocity[1]))])

    #final positions after 100 time steps when the matrix dimensions
    final_positions = []
    dimension_x = 101
    dimension_y = 103

    for i in range(0,10001):
        for pos_vel in positions_velocities:
            final_x = (pos_vel[0][0] + pos_vel[1][0] * i) % dimension_x # range is 0 to dimension_x-1
            final_y = (pos_vel[0][1] + pos_vel[1][1] * i) % dimension_y # range is 0 to dimension_y-1

            # Adjust for negative values
            if final_x < 0:
                final_x += dimension_x
            if final_y < 0:
                final_y += dimension_y

            final_positions.append((final_x, final_y))

        # Check if all positions are unique (no overlaps)
        if len(final_positions) == len(set(final_positions)):
            print(f"\n=== UNIQUE POSITIONS FOUND AT STEP {i} ===\n")

            # Draw the matrix with final positions
            matrix = [['.' for _ in range(dimension_x)] for _ in range(dimension_y)]
            for pos in final_positions:
                matrix[pos[1]][pos[0]] = '#'

            for row in matrix:
                print("".join(row))

            print(f"\n=== END OF STEP {i} ===\n")
            return i

        final_positions = []

    # Implement Part Two calculation logic heres
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
    
    print(f"=== Advent of Code 2024 - Day 14 ===")
    print()
    
    # Part One
    # print("Part One:")
    # print(f"Test input result: {solve_part_one(test_input)}")
    # print(f"Actual input result: {solve_part_one(actual_input)}")
    print()
    
    # Part Two
    print("Part Two:")
    # print(f"Test input result: {solve_part_two(test_input)}")
    print(f"Actual input result: {solve_part_two(actual_input)}")

if __name__ == "__main__":
    main()
