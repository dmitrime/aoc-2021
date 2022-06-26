use std::collections::HashMap;
use aoc2021::utils;

fn freq_map(vec: &Vec<String>) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for line in vec {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '1' => { *map.entry(i).or_insert(0) += 1; }
                _ => {}
            }
        }
    }
    map
}

fn life_support_rating(words: &Vec<String>, is_co2: bool) -> String {
    let mut idx = 0;
    let mut vec = words.clone();
    let mut map = freq_map(&vec);
    while vec.len() > 1 {
        let mut common = (map[&idx] >= vec.len() - map[&idx]) as u32;
        if is_co2 {
            common = 1 - common;
        }

        let mut new_vec: Vec<String> = Vec::<String>::new();
        for w in vec {
            if w.chars().nth(idx).unwrap().to_digit(10).unwrap() == common {
                new_vec.push(w);
            }
        }
        // println!("{} ({}): {:?}", idx, most_common, new_vec);
        vec = new_vec;
        map = freq_map(&vec);
        // println!("{:?}", map);
        idx += 1;
    }
    vec.first().unwrap().to_owned()
}

fn binary_to_int(b: &String) -> u32 {
    isize::from_str_radix(b.as_str(), 2).unwrap() as u32
}

fn main() {
    let lines = utils::read_lines("input/day-3.txt").unwrap();
    let mut vec: Vec<String> = Vec::<String>::new();

    for line in lines {
        vec.push(line.unwrap().to_owned());
    }
    let total = vec.len();
    let map = freq_map(&vec);

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..map.len() {
        if map[&i] > total - map[&i] {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            epsilon.push_str("1");
            gamma.push_str("0");
        }
    }
    let gamma_val = binary_to_int(&gamma);
    let epsilon_val = binary_to_int(&epsilon);

    println!("Part 1: {} * {} = {}", gamma_val, epsilon_val, gamma_val * epsilon_val);

    let oxygen = binary_to_int(&life_support_rating(&vec, false));
    let co2 = binary_to_int(&life_support_rating(&vec, true));
    println!("Part 2: {} * {} = {}", oxygen, co2, oxygen * co2);
}