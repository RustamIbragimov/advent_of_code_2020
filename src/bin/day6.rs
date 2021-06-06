use std::collections::HashSet;
use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/day6.txt").expect("Expected a file");
    let mut current_group_answers: HashSet<char> = HashSet::new();
    let mut total_count = 0;
    for line in content.lines() {
        if line.is_empty() {
            total_count += current_group_answers.len();
            current_group_answers.clear();
        } else {
            for c in line.chars().into_iter() {
                current_group_answers.insert(c);
            }
        }
    }
    // to account for the last group
    total_count += current_group_answers.len();
    println!("Part 1: {}", total_count);

    total_count = 0;
    current_group_answers.clear();
    // we meed to skip other people in the group if there can't be an intersection across
    // their answers. For example, person A has 'a' and person B has 'b'. It doesn't
    // make sense to look at other people's answers because there's an empty intersection
    // already.
    let mut skip_other_people = false;
    for line in content.lines() {
        if line.is_empty() {
            total_count += current_group_answers.len();
            current_group_answers.clear();
            skip_other_people = false;
        } else if !skip_other_people {
            let mut current_person_answers: HashSet<char> = HashSet::new();
            for c in line.chars().into_iter() {
                current_person_answers.insert(c);
            }
            if current_group_answers.is_empty() {
                current_group_answers.extend(&current_person_answers);
            } else {
                current_group_answers = current_group_answers
                    .intersection(&current_person_answers)
                    .copied()
                    .collect();
                if current_group_answers.is_empty() {
                    skip_other_people = true;
                }
            }
        }
    }
    // to account for the last group
    total_count += current_group_answers.len();
    current_group_answers.clear();
    println!("Part 2: {}", total_count);
}
