use crate::utils::read_input_file::read_input;

fn transform_str_to_int(line: &str) -> Vec<u8> {
    line.chars().filter(is_number).map(to_digit).collect()
}

fn is_number(x: &char) -> bool {
    let i = *x as u8;
    (48..=58).contains(&i)
}

fn to_digit(x: char) -> u8 {
    if is_number(&x) {
        x as u8 - 48
    } else {
        0
    }
}

fn get_value_from_line(line: &[u8]) -> i32 {
    let first: i32 = *line.first().unwrap_or(&0) as i32;
    let last: i32 = *line.last().unwrap_or(&0) as i32;
    first * 10 + last
}

fn change_text_numbers_to_ints(line: &str) -> String {
    let keys = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let values = [
        "z0o", "o1e", "t2o", "t3e", "f4r", "f5e", "s6x", "s7n", "e8t", "n9e",
    ];
    let mut l: String = String::from(line);
    keys.iter().enumerate().for_each(|(index, x)| {
        l = l.replace(*x, values[index]);
    });
    l
}

pub fn solution1(input: &[String]) -> i32 {
    input
        .iter()
        .map(|x| transform_str_to_int(x))
        .map(|x| get_value_from_line(&x))
        .sum()
}

pub fn solution2(input: &[String]) -> i32 {
    input
        .iter()
        .map(|x| change_text_numbers_to_ints(x))
        .map(|x| transform_str_to_int(&x))
        .map(|x| get_value_from_line(&x))
        .sum()
}

pub fn solutions() {
    let input = read_input("day1.txt");
    println!("Day 1, Solution 1: {}", solution1(&input));
    println!("Day 1, Solution 2: {}", solution2(&input));
}
