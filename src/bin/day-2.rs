use aoc2021::utils;
fn main() {
    let lines = utils::read_lines("input/day-2.txt").unwrap();
    let mut fwd = 0;
    let mut depth = 0;

    for line in lines {
        let parts: Vec<String> = line.unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let direction = &parts[0];
        let num = parts[1].parse::<i32>().unwrap();

        match direction.as_str() {
            "forward" => fwd += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => println!("Unknown direction!"),
        }
    }
    println!("Part 1: {} * {} = {}", fwd, depth, fwd * depth);

    let lines = utils::read_lines("input/day-2.txt").unwrap();
    let mut fwd = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let parts: Vec<String> = line.unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let direction = &parts[0];
        let num = parts[1].parse::<i32>().unwrap();

        match direction.as_str() {
            "forward" => {
                fwd += num;
                depth += aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => println!("Unknown direction!"),
        }
    }
    println!("Part 2: {} * {} = {}", fwd, depth, fwd * depth);
}