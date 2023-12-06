use std::collections::HashMap;
use std::fs::read_to_string;

fn part1(filename: &str) -> i32 {
    let mut total: i32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        let line = line.replace(|c: char| !c.is_numeric(), "");

        let num = format!(
            "{}{}",
            line.chars().next().unwrap(),
            line.chars().last().unwrap()
        );

        total += num.parse::<i32>().unwrap();
    }

    total
}

fn part2(filename: &str) -> i32 {
    let mut total: i32 = 0;

    let map: HashMap<&str, &str> = HashMap::from([
        ("one", "1e"),
        ("two", "2o"),
        ("three", "3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "8t"),
        ("nine", "9e"),
    ]);

    for line in read_to_string(filename).unwrap().lines() {
        let mut line = line.to_string();

        for (key, value) in &map {
            line = line.replace(key, value);
        }

        line = line.replace(|c: char| !c.is_numeric(), "");

        let num = format!(
            "{}{}",
            line.chars().next().unwrap(),
            line.chars().last().unwrap()
        );

        total += num.parse::<i32>().unwrap();
    }

    total
}

fn main() {
    let filename = "input.txt";

    println!("{}", part1(filename));
    println!("{}", part2(filename));
}
