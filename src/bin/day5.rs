use std::fs;

fn get_seat_ids(lines: &Vec<&str>) -> Vec<i32> {
    let mut seat_ids: Vec<i32> = Vec::new();
    for line in lines {
        let line_chars: Vec<_> = line.chars().collect();
        let (mut row_upper, mut row_lower, mut col_upper, mut col_lower) = (127, 0, 7, 0);
        for i in 0..7 {
            let mid = (row_upper + row_lower) / 2;
            if line_chars[i] == 'F' {
                row_upper = mid;
            } else {
                row_lower = mid + 1;
            }
        }
        assert_eq!(row_lower, row_upper);
        for i in 7..10 {
            let mid = (col_upper + col_lower) / 2;
            if line_chars[i] == 'L' {
                col_upper = mid;
            } else {
                col_lower = mid + 1;
            }
        }
        assert_eq!(col_lower, col_upper);

        seat_ids.push(row_lower * 8 + col_lower);
    }
    seat_ids
}

fn find_missing_seat_id(seat_ids: &mut Vec<i32>) -> Option<i32> {
    seat_ids.sort();
    for i in 0..seat_ids.len() - 1 {
        if seat_ids[i + 1] - seat_ids[i] > 1 {
            return Some(seat_ids[i] + 1);
        }
    }
    None
}

fn main() {
    let content = fs::read_to_string("inputs/day5.txt").expect("Expected a file");
    let lines: Vec<_> = content.lines().collect();
    let mut seat_ids = get_seat_ids(&lines);
    let highest_seat_id = seat_ids.iter().max().unwrap();

    println!("Part 1: {}", highest_seat_id);

    let missing_seat_id = find_missing_seat_id(&mut seat_ids);
    match missing_seat_id {
        None => println!("Shouldn't happen"),
        Some(missing_seat_id) => println!("Part 2: {}", missing_seat_id),
    }
}
