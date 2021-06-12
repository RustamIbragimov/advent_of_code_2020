use std::fs;


type Layout = Vec<Vec<char>>;


struct SimulationResult {
    is_changed: bool,
    new_layout: Layout,
}

fn get_updated_seat(layout: &mut Layout, i: usize, j: usize, occupied_adj_seats_cnt: i32) -> (char, bool) {
    if layout[i][j] == 'L' && occupied_adj_seats_cnt == 0 {
        ('#', true)
    } else if layout[i][j] == '#' && occupied_adj_seats_cnt >= 4 {
        ('L', true)
    } else {
        (layout[i][j], false)
    }
}

fn simulate_part1(layout: &Layout) -> SimulationResult {
    let (n, m) = (layout.len(), layout[0].len());
    let dirs = vec![
        (1, 1), (-1, -1), 
        (1, -1), (-1, 1),
        (0, 1), (1, 0),
        (0, -1), (-1, 0),
    ];
    let mut new_layout = vec![vec!['!'; m]; n]; 
    let mut is_changed = false;
    for i in 0..n {
        for j in 0..m {
            let mut occupied_adj_seats_cnt = 0;
            for (di, dj) in &dirs {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 && layout[ni as usize][nj as usize] == '#' {
                    occupied_adj_seats_cnt += 1;
                }
            }
            if layout[i][j] == 'L' && occupied_adj_seats_cnt == 0 {
                new_layout[i][j] = '#';
                is_changed = true;
            } else if layout[i][j] == '#' && occupied_adj_seats_cnt >= 4 {
                new_layout[i][j] = 'L';
                is_changed = true;
            } else {
                new_layout[i][j] = layout[i][j];
            }
        }
    }
    SimulationResult { is_changed, new_layout }
}

fn simulate_part2(layout: &Layout) -> SimulationResult {
    let (n, m) = (layout.len(), layout[0].len());
    let dirs = vec![
        (1, 1), (-1, -1), 
        (1, -1), (-1, 1),
        (0, 1), (1, 0),
        (0, -1), (-1, 0),
    ];
    let mut new_layout = vec![vec!['!'; m]; n]; 
    let mut is_changed = false;
    for i in 0..n {
        for j in 0..m {
            let mut occupied_adj_seats_cnt = 0;
            for (di, dj) in &dirs {
                let (mut ni, mut nj) = (i as i32, j as i32);
                loop {
                    ni = ni + di;
                    nj = nj + dj;
                    if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                        if layout[ni as usize][nj as usize] == '#' {
                            occupied_adj_seats_cnt += 1;
                            break;
                        } else if layout[ni as usize][nj as usize] == 'L' {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            if layout[i][j] == 'L' && occupied_adj_seats_cnt == 0 {
                new_layout[i][j] = '#';
                is_changed = true;
            } else if layout[i][j] == '#' && occupied_adj_seats_cnt >= 5 {
                new_layout[i][j] = 'L';
                is_changed = true;
            } else {
                new_layout[i][j] = layout[i][j];
            }
        }
    }

    SimulationResult { new_layout, is_changed }
}

fn part1(content: &str) {
    let mut layout: Layout = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    loop {
        let result = simulate_part1(&mut layout);
        if !result.is_changed {
            break;
        } else {
            layout = result.new_layout;
        }
    }
    let part1 = layout
        .into_iter()
        .flatten()
        .filter(|c| *c == '#')
        .count();

    println!("Part 1 {}", part1);
}

fn part2(content: &str) {
    let mut layout: Layout = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    loop {
        let result = simulate_part2(&mut layout);
        if !result.is_changed {
            break;
        } else {
            layout = result.new_layout;
        }
    }
    let part2 = layout
        .into_iter()
        .flatten()
        .filter(|c| *c == '#')
        .count();

    println!("Part 2 {}", part2);
}

fn main() {
    let content = fs::read_to_string("inputs/day11.txt").expect("Expected a file");
    part1(&content);
    part2(&content);
}

