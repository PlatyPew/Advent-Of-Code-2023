use std::collections::HashMap;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn strip(line: &str) -> Vec<&str> {
    let line: &str = line.split(": ").nth(1).unwrap();
    let rolls: Vec<&str> = line.split("; ").collect::<Vec<&str>>();
    rolls
}

fn parse(roll: &str) -> HashMap<&str, i32> {
    let mut gacha: HashMap<&str, i32> = HashMap::new();

    let dice: Vec<&str> = roll.split(", ").collect::<Vec<&str>>();

    for die in dice.iter() {
        let colour: Vec<&str> = die.split(" ").collect::<Vec<&str>>();
        gacha.insert(colour[1], colour[0].parse::<i32>().unwrap());
    }

    gacha
}

fn check(gacha: HashMap<&str, i32>) -> bool {
    if gacha.contains_key("red") && *gacha.get("red").unwrap() > RED {
        return false;
    }

    if gacha.contains_key("green") && *gacha.get("green").unwrap() > GREEN {
        return false;
    }

    if gacha.contains_key("blue") && *gacha.get("blue").unwrap() > BLUE {
        return false;
    }

    true
}

fn part1(filename: &str) -> i32 {
    let mut total: i32 = 0;

    for (index, line) in std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
    {
        total = total + (index + 1) as i32;

        let rolls: Vec<&str> = strip(line);

        for roll in rolls.iter() {
            let gacha: HashMap<&str, i32> = parse(roll);
            if !check(gacha) {
                total = total - (index + 1) as i32;
                break;
            }
        }
    }

    total
}

fn part2(filename: &str) -> i32 {
    let mut total: i32 = 0;
    for line in std::fs::read_to_string(filename).unwrap().lines() {
        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;

        let rolls: Vec<&str> = strip(line);
        for roll in rolls.iter() {
            let gacha: HashMap<&str, i32> = parse(roll);

            if gacha.contains_key("red") && *gacha.get("red").unwrap() > max_red {
                max_red = *gacha.get("red").unwrap();
            }

            if gacha.contains_key("green") && *gacha.get("green").unwrap() > max_green {
                max_green = *gacha.get("green").unwrap();
            }

            if gacha.contains_key("blue") && *gacha.get("blue").unwrap() > max_blue {
                max_blue = *gacha.get("blue").unwrap();
            }
        }

        total += max_red * max_green * max_blue;
    }

    total
}

fn main() {
    println!("{}", part1("input.txt"));
    println!("{}", part2("input.txt"));
}
