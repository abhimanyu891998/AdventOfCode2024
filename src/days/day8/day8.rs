// Advent of Code 2024 - Day 8
// Problem link: https://adventofcode.com/2024/day/8

pub fn solve() {
    println!("Solution for Day 8");
    let DAY = 8;
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

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct point {
    x: i32,
    y: i32,
}

fn calculate_part_one_result(input: &str) -> String {
    // Implement Part One calculation logic here
    let mut antenna_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antenna_points_to_points_dict: std::collections::HashMap<char, std::collections::HashSet<point>> = std::collections::HashMap::new();

    for (y, row) in antenna_grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch.is_alphabetic() || ch.is_numeric() {
                antenna_points_to_points_dict
                    .entry(ch)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(point { x: x as i32, y: y as i32 });
            }
        }
    }

    let mut all_unique_points: std::collections::HashSet<point> = std::collections::HashSet::new();
    for (antenna, points) in &antenna_points_to_points_dict {
        //Select pairs of points    
        for point1 in points {
            for point2 in points {
                if point1 == point2 {
                    continue;
                }

                
                // find the coordinate that would make point1 as the mid point of the line going from point1 and point2
                let x_diff = point2.x - point1.x;
                let y_diff = point2.y - point1.y;

                let x_diff_abs = x_diff.abs();
                let y_diff_abs = y_diff.abs();


                let slope = y_diff as f64 / x_diff as f64;

                let mut x_sign = 1;
                let mut y_sign = 1;

                //potential antinode for point1 as the midpoint
                if(point1.x >= point2.x){
                    x_sign = 1;
                }
                else{
                    x_sign = -1;
                }

                if(point1.y >= point2.y){
                    y_sign = 1;
                }
                else{
                    y_sign = -1;
                }

                let potential_antinode_1 = point { x: point1.x + x_sign * x_diff_abs, y: point1.y + y_sign * y_diff_abs };

                //Check if the potential antinode is on the grid
                if potential_antinode_1.x >= 0 && potential_antinode_1.x < antenna_grid[0].len() as i32 && potential_antinode_1.y >= 0 && potential_antinode_1.y < antenna_grid.len() as i32 {
                    all_unique_points.insert(potential_antinode_1);
                }


                //ponteial antinode for point2 as the midpoint
                if(point2.x >= point1.x){
                    x_sign = 1;
                }
                else{
                    x_sign = -1;
                }

                if(point2.y >= point1.y){
                    y_sign = 1;
                }
                else{
                    y_sign = -1;
                }

                let potential_antinode_2 = point { x: point2.x + x_sign * x_diff_abs, y: point2.y + y_sign * y_diff_abs };

                if potential_antinode_2.x >= 0 && potential_antinode_2.x < antenna_grid[0].len() as i32 && potential_antinode_2.y >= 0 && potential_antinode_2.y < antenna_grid.len() as i32 {
                    all_unique_points.insert(potential_antinode_2);
                }  
            }
        }
    }

    return all_unique_points.len().to_string();
}

fn find_points_on_line_that_passes_through_1_and_2(point1: &point, point2: &point, antenna_grid: &Vec<Vec<char>>) -> std::collections::HashSet<point> {
    let mut points_on_line: std::collections::HashSet<point> = std::collections::HashSet::new();
    for y in 0..antenna_grid.len() as i32 {
        for x in 0..antenna_grid[0].len() as i32 {
            let dx1 = x - point1.x;
            let dy1 = y - point1.y;
            let dx2 = point2.x - point1.x;
            let dy2 = point2.y - point1.y;

            if dx2 != 0 && dy2 != 0 {
                if (dy1 as f64 / dy2 as f64) == (dx1 as f64 / dx2 as f64) {
                    points_on_line.insert(point { x: x as i32, y: y as i32 });
                }
            }
        }
    }

    return points_on_line;
}

fn calculate_part_two_result(input: &str) -> String {
    // Implement Part Two calculation logic here
      // Implement Part One calculation logic here
      let mut antenna_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
      let mut antenna_points_to_points_dict: std::collections::HashMap<char, std::collections::HashSet<point>> = std::collections::HashMap::new();
  
      for (y, row) in antenna_grid.iter().enumerate() {
          for (x, &ch) in row.iter().enumerate() {
              if ch.is_alphabetic() || ch.is_numeric() {
                  antenna_points_to_points_dict
                      .entry(ch)
                      .or_insert_with(std::collections::HashSet::new)
                      .insert(point { x: x as i32, y: y as i32 });
              }
          }
      }
  
      let mut all_unique_points: std::collections::HashSet<point> = std::collections::HashSet::new();
      for (antenna, points) in &antenna_points_to_points_dict {
        let points_vec: Vec<&point> = points.iter().collect();
        for i in 0..points_vec.len() {
            for j in i+1..points_vec.len() {
                let point1 = points_vec[i];
                let point2 = points_vec[j];
                let current_unique_points = find_points_on_line_that_passes_through_1_and_2(point1, point2, &antenna_grid);
                all_unique_points = all_unique_points.union(&current_unique_points).cloned().collect();
            }
        }
      }
        return all_unique_points.len().to_string();
}

