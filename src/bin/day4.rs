use std::fs;
use std::collections::{HashMap, HashSet};

struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}


impl<'a> Passport<'a> {
    fn validate_required_fields(&self) -> bool {
        let required_fields: HashSet<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
            .into_iter()
            .collect();
        let keys: HashSet<_> = self.fields.keys().cloned().collect();
        let found_fields: Vec<_> = keys.intersection(&required_fields).collect();
        if found_fields.len() == 8 {
            true
        } else if found_fields.len() == 7 && !keys.contains(&"cid") {
            true
        } else {
            false
        }
    }

    fn validate_fields_format(&self) -> bool {
        if !is_n_digit_number(self.fields["byr"], 4) || !is_number_in_range(self.fields["byr"], 1920, 2002) {
            return false;
        }
        if !is_n_digit_number(self.fields["iyr"], 4) || !is_number_in_range(self.fields["iyr"], 2010, 2020) {
            return false;
        }
        if !is_n_digit_number(self.fields["eyr"], 4) || !is_number_in_range(self.fields["eyr"], 2020, 2030) {
            return false;
        }
        if !is_valid_height(self.fields["hgt"]) {
            return false;
        }
        if !is_valid_hair_color(self.fields["hcl"]) {
            return false;
        }
        if !is_valid_eye_color(self.fields["ecl"]) {
            return false;
        }
        if !is_n_digit_number(self.fields["pid"], 9) {
            return false;
        }
        true
    }
}

fn is_valid_hair_color(color: &str) -> bool {
    if !color.len() == 7 || !color.starts_with("#") {
        return false;
    }
    color[1..].chars().all(|c| c.is_lowercase() || c.is_numeric())
}

fn is_valid_height(height: &str) -> bool {
    let in_cm = height.ends_with("cm");
    let in_in = height.ends_with("in");
    if !in_cm && !in_in {
        return false;
    }
    let value: &str;
    if in_cm {
        value = height.split("cm").collect::<Vec<_>>()[0];
    } else {
        value = height.split("in").collect::<Vec<_>>()[0];
    }

    match value.parse::<i32>() {
        Ok(value) => {
            if in_cm {
                value >= 150 && value <= 193
            } else {

                value >= 59 && value <= 76
            }
        }
        Err(_) => false
    }
}

fn is_valid_eye_color(color: &str) -> bool {
    let valid_colors: HashSet<_> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().collect();
    valid_colors.contains(color)
}

fn is_n_digit_number(s: &str, expected_digits_count: usize) -> bool {
    match s.parse::<i32>() {
        Ok(_) => {
            s.len() == expected_digits_count
        }
        Err(_) => false
    }
}

fn is_number_in_range(s: &str, from: i32, to: i32) -> bool {
    match s.parse::<i32>() {
        Ok(number) => {
            number >= from && number <= to
        }
        Err(_) => false
    }
}

fn main() {
    let content = fs::read_to_string("inputs/day4.txt").expect("Expected a file");
    let mut passports: Vec<Passport> = Vec::new();
    let mut current_passport_fields: HashMap<&str, &str> = HashMap::new();

    for line in content.lines() {
        if line.is_empty() {
            passports.push(Passport { fields: current_passport_fields });
            current_passport_fields = HashMap::new();
        } else {
            let items = line.split(" ");
            for item in items {
                let key_value: Vec<_> = item.split(":").collect();
                current_passport_fields.insert(key_value[0], key_value[1]);
            }
        }
    }
    if !current_passport_fields.is_empty() {
        passports.push(Passport { fields: current_passport_fields });
    }

    let (mut part_1_valid_count, mut part_2_valid_count) = (0, 0);
    for passport in passports {
        if passport.validate_required_fields() {
            part_1_valid_count += 1;
            if passport.validate_fields_format() {
                part_2_valid_count += 1;
            }
        }
    }

    println!("Part 1 {}", part_1_valid_count);
    println!("Part 2 {}", part_2_valid_count)
}