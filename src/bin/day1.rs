use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").expect("Expected a file");
    let numbers: Vec<i32> = contents
        .lines()
        .map(|s| s.parse().expect("Couldn't parse a string into an int"))
        .collect();

    let n = numbers.len();
    'outer: for i in 0..n {
        for j in i + 1..n {
            if numbers[i] + numbers[j] == 2020 {
                println!(
                    "Part 1: {first} + {second} = 2020, and {first} * {second} = {result}",
                    first = numbers[i], second = numbers[j], result = numbers[i] * numbers[j]
                );
                break 'outer;
            }
        }
    }

    'outer: for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!(
                        "Part 2: {first} + {second} + {third} = 2020, and {first} * {second} * {third} = {result}",
                        first = numbers[i], second = numbers[j], third = numbers[k], result = numbers[i] * numbers[j] * numbers[k]
                    );
                    break 'outer;
                }
            }
        }
    }
}
