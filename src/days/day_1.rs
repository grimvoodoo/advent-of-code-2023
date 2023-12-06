use regex::Regex;
use std::fs::read_to_string;

pub fn task_1() {
    let file_path = "data/1-1";
    println!("In File{}", file_path);
    let mut count: i32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let re = Regex::new(r"\d").unwrap();
        let matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let first_number = matches.first().unwrap();
        let last_number = matches.last().unwrap();
        let mut sum: String = String::new();
        sum.push_str(first_number);
        sum.push_str(last_number);
        let loop_count: i32 = sum.parse().unwrap();
        count += loop_count;
    }
    println!("{:?}", count)
}

pub fn task_2() {
    let file_path = "data/1-1";
    println!("In File{}", file_path);
    let mut count: i32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let mut number_list: Vec<&str> = Vec::new();
        let re = Regex::new(r"\b(one|two|three|four|five|six|seven|eight|nine|\d)\b").unwrap();

        // Had to add in some wonky checking here as the regex crate does not support forward looking.
        for i in 0..line.len() {
            for j in (i + 1)..=line.len() {
                let substring = &line[i..j];
                if let Some(captures) = re.captures(substring) {
                    let matched_text = captures.get(0).unwrap().as_str();
                    match matched_text {
                        "one" => number_list.push("1"),
                        "two" => number_list.push("2"),
                        "three" => number_list.push("3"),
                        "four" => number_list.push("4"),
                        "five" => number_list.push("5"),
                        "six" => number_list.push("6"),
                        "seven" => number_list.push("7"),
                        "eight" => number_list.push("8"),
                        "nine" => number_list.push("9"),
                        _ => number_list.push(matched_text),
                    }
                }
            }
        }

        let first_number = number_list.first().unwrap();
        let last_number = number_list.last().unwrap();
        let mut sum: String = String::new();
        sum.push_str(first_number);
        sum.push_str(last_number);
        let loop_count: i32 = sum.parse().unwrap();
        count += loop_count;
    }
    println!("{:?}", count);
}
