use std::fs;
use regex::Regex;

pub fn day3() {
    let mut contents = fs::read_to_string("./inputs/day3.txt")
        .expect("Something went wrong");

    let regex_pattern = r"mul\((\d+),(\d+)\)";
    let regex_pattern = Regex::new(regex_pattern).unwrap();

    let regex_not_allowed_pattern = r"don't\((.*?)do\(\)";
    let regex_not_allowed_pattern = Regex::new(regex_not_allowed_pattern).unwrap();

    let regex_last_dont_pattern = r"don't\(\).*";
    let regex_last_dont_pattern = Regex::new(&regex_last_dont_pattern).unwrap();

    contents = regex_not_allowed_pattern.replace_all(&contents, "").to_string();
    contents = regex_last_dont_pattern.replace_all(&contents, "").to_string();
    println!("{}", contents);


    let mut sum: i32 = 0;

    for pattern_match in regex_pattern.captures_iter(&contents) {
      let first_number: i32 = pattern_match[1].parse().unwrap();
      let second_number: i32 = pattern_match[2].parse().unwrap();

      sum += first_number * second_number;

    }

    println!("{}", sum);
} 