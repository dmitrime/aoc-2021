use aoc2021::utils;

fn get_fuel(pos: i32, vec: &Vec<i32>) -> i32 {
    vec.iter().map(|v| (pos-v).abs()).sum()
}

fn seq_sum(a: i32, b: i32) -> i32 {
    let n = (a-b).abs();
    n * (n+1) / 2
}

fn get_fuel2(pos: i32, vec: &Vec<i32>) -> i32 {
    vec.iter().map(|v| seq_sum(pos, *v)).sum()
}

fn main() {
    let positions = utils::read_ints("input/day-7.txt", ",");
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let min_fuel = (*min..=*max).map(|u| get_fuel(u, &positions)).min().unwrap();
    println!("Part 1: {}", min_fuel);

    let min_fuel  = (*min..=*max).map(|u| get_fuel2(u, &positions)).min().unwrap();
    println!("Part 2: {}", min_fuel);
}