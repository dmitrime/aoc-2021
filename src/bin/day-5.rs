use aoc2021::utils;
use std::cmp::{ min, max };

fn main() {
    let mut board: Vec<Vec<i32>> = Vec::new();
    let mut board2: Vec<Vec<i32>> = Vec::new();
    for _ in 0..1000 {
        board.push(vec![0; 1000]);
        board2.push(vec![0; 1000]);
    }

    let lines = utils::read_lines("input/day-5.txt").unwrap();
    for line in lines.map(|l| l.unwrap()) {
        let mut ln = line.split(" -> ");
        let (x1, y1) = ln.next().unwrap().split_once(',').unwrap();
        let (x2, y2) = ln.next().unwrap().split_once(',').unwrap();

        let (x1, y1) = (x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap());
        let (x2, y2) = (x2.parse::<usize>().unwrap(), y2.parse::<usize>().unwrap());

        if x1 == x2 || y1 == y2 {
            for i in min(y1, y2)..=max(y1, y2) {
                for j in min(x1, x2)..=max(x1, x2) {
                    board[i][j] += 1;
                    board2[i][j] += 1;
                }
            }
        } else {
            let steps = max(x1, x2) - min(x1, x2);
            if x1 < x2 && y1 < y2 {
                for i in 0..=steps {
                    board2[y1+i][x1+i] += 1;
                }
            } else if x1 < x2 && y1 > y2 {
                for i in 0..=steps {
                    board2[y1-i][x1+i] += 1;
                }
            } else if x1 > x2 && y1 < y2 {
                for i in 0..=steps {
                    board2[y1+i][x1-i] += 1;
                }
            } else if x1 > x2 && y1 > y2 {
                for i in 0..=steps {
                    board2[y1-i][x1-i] += 1;
                }
            }
        }
    }
    let mut intersect = 0;
    for b in board {
        intersect += b.iter().filter(|&x| *x > 1).count();
    }
    println!("Part 1: {}", intersect);

    let mut intersect2 = 0;
    for b in board2 {
        intersect2 += b.iter().filter(|&x| *x > 1).count();
    }
    println!("Part 2: {}", intersect2);
}