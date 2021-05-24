use std::fs;

fn count_trees_when_traversing(map: &Vec<Vec<char>>, right_steps: usize, down_steps: usize) -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut count = 0;
    let x_size = map[0].len();
    while y < map.len() {
        if map[y][x] == '#' {
            count += 1;
        }
        x = (x + right_steps) % x_size;
        y += down_steps;
    }
    count
}

fn main() {
    let content = fs::read_to_string("inputs/day3.txt").expect("Expected a file");
    let map: Vec<Vec<_>> = content.lines().map(|s| s.chars().collect()).collect();

    let part_1_count = count_trees_when_traversing(&map, 3, 1);
    println!("Part 1: {}", part_1_count);

    let mut part_2_count: i64 = 1;
    let patterns = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (right_steps, down_steps) in patterns {
        part_2_count *= count_trees_when_traversing(&map, right_steps, down_steps) as i64;
    }
    println!("Part 2: {}", part_2_count)
}