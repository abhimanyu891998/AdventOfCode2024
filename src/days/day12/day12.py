# Advent of Code 2024 - Day 12
# Problem link: https://adventofcode.com/2024/day/12

import os
from tkinter import N


def get_neighbors(character_matrix, r, c, non_neighbor_rows, non_neighbor_columns):
    #if vertical or horizontal values are same as current val, then they are neighbors
    neighbors = []
    non_neighbors = []
    current_char = character_matrix[r][c]

    if r-1 >=0 and character_matrix[r-1][c] == current_char:
        neighbors.append((r-1, c))
    else:
        non_neighbors.append((r-1, c))
    if r+1 < len(character_matrix) and character_matrix[r+1][c] == current_char:
        neighbors.append((r+1, c))
    else:
        non_neighbors.append((r+1, c))
    if c-1 >=0 and character_matrix[r][c-1] == current_char:
        neighbors.append((r, c-1))
    else:
        non_neighbors.append((r, c-1))
    if c+1 < len(character_matrix[0]) and character_matrix[r][c+1] == current_char:
        neighbors.append((r, c+1))
    else:
       non_neighbors.append((r, c+1))

    return neighbors, non_neighbors



def bfs(character_matrix, visited_matrix, r, c, is_part_two = False):
    queue = [(r, c)]
    visited_matrix[r][c] = True

    area = 0
    perimeter = 0
    non_neighbor_rows = set()
    non_neighbor_columns = set()
    convex_corners = 0
    concave_corners = 0
    while len(queue) > 0:
        current_char = queue.pop(0)
        current_r, current_c = current_char
        r, c = current_r, current_c
        neighbors, non_neighbors  = get_neighbors(character_matrix, current_r, current_c, non_neighbor_rows, non_neighbor_columns)
        for neighbor in neighbors:
            if visited_matrix[neighbor[0]][neighbor[1]] == False:
                queue.append(neighbor)
                visited_matrix[neighbor[0]][neighbor[1]] = True
        if not is_part_two:
            perimeter = perimeter + (4-len(neighbors))
        area = area + 1

        if is_part_two:
            #right-bottom
            current_char_value = character_matrix[current_char[0]][current_char[1]]
            if r-1 >=0 and c+1 < len(character_matrix[0]) and (r-1,c) in neighbors and (r,c+1) in neighbors and character_matrix[r-1][c+1] != current_char_value:
                convex_corners = convex_corners + 1
            #right-bottom
            if r+1 < len(character_matrix) and c+1 < len(character_matrix[0]) and (r+1,c) in neighbors and (r,c+1) in neighbors and character_matrix[r+1][c+1] != current_char_value:
                convex_corners = convex_corners + 1
            #left-top
            if r-1 >=0 and c-1 >=0 and (r-1,c) in neighbors and (r,c-1) in neighbors and character_matrix[r-1][c-1] != current_char_value:
                convex_corners = convex_corners + 1
            #left-bottom
            if r+1 < len(character_matrix) and c-1 >=0 and (r+1,c) in neighbors and (r,c-1) in neighbors and character_matrix[r+1][c-1] != current_char_value:
                convex_corners = convex_corners + 1
            #concave corners
            if (r-1<0 or (r-1,c) in non_neighbors) and (c+1>=len(character_matrix[0]) or (r,c+1) in non_neighbors):
                concave_corners = concave_corners + 1
            if (r+1>=len(character_matrix[1]) or (r+1,c) in non_neighbors) and (c+1>=len(character_matrix[0]) or (r,c+1) in non_neighbors):
                concave_corners = concave_corners + 1
            if (r-1<0 or (r-1,c) in non_neighbors) and (c-1<0 or (r,c-1) in non_neighbors):
                concave_corners = concave_corners + 1
            if (r+1>=len(character_matrix[1]) or (r+1,c) in non_neighbors) and (c-1<0 or (r,c-1) in non_neighbors):
                concave_corners = concave_corners + 1


            perimeter = convex_corners + concave_corners
            

    return area, perimeter


def solve_part_one(input_data):
    """
    Solve part one of the problem.
    """
    character_matrix = [[char for char in line ] for line in input_data.split("\n")]
    visited_matrix = [[False for _ in range(len(character_matrix[0]))] for _ in range(len(character_matrix))]
    final_sum = 0

    for r in range(len(character_matrix)):
        for c in range(len(character_matrix[r])):
            if visited_matrix[r][c] == False:
                area, perimeter = bfs(character_matrix, visited_matrix, r, c, False)
                #Print area and perimeter
                final_sum = final_sum + area * perimeter

    return final_sum 

def solve_part_two(input_data):
    """
    Solve part two of the problem.
    """
    character_matrix = [[char for char in line ] for line in input_data.split("\n")]
    visited_matrix = [[False for _ in range(len(character_matrix[0]))] for _ in range(len(character_matrix))]
    final_sum = 0

    for r in range(len(character_matrix)):
        for c in range(len(character_matrix[r])):
            if visited_matrix[r][c] == False:
                area, perimeter = bfs(character_matrix, visited_matrix, r, c, True)
                final_sum = final_sum + area * perimeter

    return final_sum 

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
    
    print(f"=== Advent of Code 2024 - Day 12 ===")
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
