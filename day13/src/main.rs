use regex::Regex;
use std::fs;

fn main() {
    println!("Hello, world!");
}

fn part1(path: &str) {
    let button_re = Regex::new(r"Button \w: \w\+(\d+), \w\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: \w=(\d+), \w=(\d+)").unwrap();

    let input = fs::read_to_string(path).unwrap();

    let mut machines = Vec::new();
    for lines in input.lines().collect::<Vec<&str>>().chunks(4) {
        let a_captures = button_re.captures(lines[0]).unwrap();
        let b_captures = button_re.captures(lines[1]).unwrap();
        let prize_captures = prize_re.captures(lines[2]).unwrap();
        machines.push((
            (
                a_captures[1].to_string().parse::<i32>().unwrap(),
                a_captures[2].to_string().parse::<i32>().unwrap(),
            ),
            (
                b_captures[1].to_string().parse::<i32>().unwrap(),
                b_captures[2].to_string().parse::<i32>().unwrap(),
            ),
            (
                prize_captures[1].to_string().parse::<i32>().unwrap(),
                prize_captures[2].to_string().parse::<i32>().unwrap(),
            ),
        ));
    }
}
