use std::collections::HashMap;

use aoc2021::utils;

fn count_fish(timer: &mut Vec<i32>, days: u32) -> usize {
    for _ in 0..days {
        let mut add: Vec<i32> = Vec::new();
        for i in 0..timer.len() {
            if timer[i] == 0 {
                add.push(8);
                timer[i] = 6;
            } else {
                timer[i] -= 1;
            }
        }
        timer.extend(add);
    }
    timer.len()
}

fn count_fish_map(timer: &Vec<i32>, days: u32) -> usize {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for t in timer {
        *map.entry(*t).or_insert(0) += 1;
    }
    for _ in 0..days {
        let mut new: HashMap<i32, usize> = HashMap::new();
        for (time,count) in &map {
            if *time > 0 {
                *new.entry(time - 1).or_insert(0) += count;
            } else {
                *new.entry(8).or_insert(0) += count;
                *new.entry(6).or_insert(0) += count;
            }
        }
        map = new;
    }
    let mut total = 0;
    for (_, count) in map {
        total += count;
    }
    total
}
fn main() {
    let timer = utils::read_ints("input/day-6.txt", ",");
    
    println!("Part 1: {}", count_fish(&mut timer.clone(), 80));
    println!("Part 1 (map): {}", count_fish_map(&timer.clone(), 80));
    println!("Part 2 (map): {}", count_fish_map(&timer.clone(), 256));
}