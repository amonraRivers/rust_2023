use crate::utils::read_input_file::read_input;

fn check_set(set: &str) -> bool {
    let games = set.trim().split(';');
    let mut res = true;
    for game in games {
        if !check_game(game) {
            res = false;
            break;
        }
    }
    res
}

fn check_game(game: &str) -> bool {
    let keys = ["red", "green", "blue"];
    let values = [12, 13, 14];
    let cubes_i = game.trim().split(',');
    let mut res = true;
    for cubes in cubes_i {
        let cube: Vec<&str> = cubes.trim().split(' ').collect();
        let val = cube[0].parse::<i32>().unwrap();
        let key = cube[1];
        let i = keys.iter().position(|&x| x == key).unwrap();
        if val > values[i] {
            res = false;
            break;
        }
    }
    res
}

fn solution1(input: &[String]) -> i32 {
    let mut sum: i32 = 0;
    for (index, line) in input.iter().enumerate() {
        let o_set = line.split(':').last();
        if let Some(set) = o_set {
            if check_set(set) {
                sum += i32::try_from(index).unwrap() + 1;
            }
        }
    }
    sum
}

fn get_set_power(set: &str) -> i32 {
    let games = set.trim().split(';');
    let mut max = [0; 3];
    for game in games {
        let game_values = get_game_values(game);
        for i in 0..3 {
            if max[i] < game_values[i] {
                max[i] = game_values[i];
            }
        }
    }
    max[0] * max[1] * max[2]
}

fn get_game_values(game: &str) -> [i32; 3] {
    let keys = ["red", "green", "blue"];
    let cubes_i = game.trim().split(',');
    let mut res = [0; 3];
    for cubes in cubes_i {
        let cube: Vec<&str> = cubes.trim().split(' ').collect();
        let val = cube[0].parse::<i32>().unwrap();
        let key = cube[1];
        let i = keys.iter().position(|&x| x == key).unwrap();
        res[i] = val;
    }
    res
}

fn solution2(input: &[String]) -> i32 {
    let mut sum: i32 = 0;
    for (_index, line) in input.iter().enumerate() {
        let o_set = line.split(':').last();
        if let Some(set) = o_set {
            sum += get_set_power(set);
        }
    }
    sum
}

pub fn solutions() {
    println!("Day 2");
    let input = read_input("day2.txt");
    println!("Solution 1 {}", solution1(&input));
    println!("Solution 2 {}", solution2(&input));
}
