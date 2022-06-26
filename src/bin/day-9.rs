use aoc2021::utils;

fn is_lowest(x: usize, y: usize, m: &Vec<Vec<u8>>) -> bool {
    let p: u8 = m[y][x];
    if x+1 < m[0].len() && p >= m[y][x+1] {
        return false
    } else if (x as i32)-1 >= 0 && p >= m[y][x-1] {
        return false
    } else if y+1 < m.len() && p > m[y+1][x] {
        return false
    } else if (y as i32)-1 >= 0 && p > m[y-1][x] {
        return false
    }
    true
}

fn low_points(m: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut low: Vec<u8> = Vec::new();
    for y in 0..m.len() {
        for x in 0..m[0].len() {
            if is_lowest(x, y, &m) {
                low.push(m[y][x]);
            }
        }
    }
    low
}

fn main() {
    let mut hmap: Vec<Vec<u8>> = Vec::new();

    let lines = utils::read_lines("input/day-9.txt").unwrap();
    for line in lines.map(|l| l.unwrap()) {
        let mut vec: Vec<u8> = Vec::new();
        for c in line.chars() {
            vec.push(c.to_digit(10).unwrap() as u8);
        }
        hmap.push(vec);
    }
    
    let point_sum: u32 = low_points(&hmap).iter().map(|p| (p + 1) as u32).sum();
    println!("Part 1: {}", point_sum);

}