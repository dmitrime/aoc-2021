use aoc2021::utils;

fn parse_numbers(line: String, is_comma: bool) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let sep = if is_comma { "," } else { " " };
    for n in line.split(sep) {
        if n.len() > 0 {
            numbers.push(n.parse::<i32>().unwrap());
        }
    }
    numbers
}

fn mark(n: i32, board: &mut Vec<Vec<i32>>) {
    for b in board {
        for i in 0..b.len() {
            if b[i] == n {
                b[i] = -b[i];
                if n == 0 {
                    b[i] = -1000;
                }
            }
        }
    }
}

fn unmarked(b: usize, board: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if board[b*5 + i][j] > 0 {
                sum += board[b*5 +i][j];
            }
        }
    }
    sum
}

fn check_bingo(unfinished: &Vec<usize>, board: &Vec<Vec<i32>>) -> Vec<usize> {
    let mut ans: Vec<usize> = Vec::new();
    for b in unfinished {
        for i in 0..5 {
            let mut is_neg = true;
            for j in 0..5 {
                if board[b*5 + i][j] > 0 {
                    is_neg = false;
                }
            }
            if is_neg {
                ans.push(*b);
            }
        }
        for i in 0..5 {
            let mut is_neg = true;
            for j in 0..5 {
                if board[b*5 + j][i] > 0 {
                    is_neg = false;
                }
            }
            if is_neg {
                ans.push(*b);
            }
        }
    }
    ans
}

fn main() {
    let mut lines = utils::read_lines("input/day-4.txt").unwrap();
    let first = lines.next().unwrap();
    let numbers = parse_numbers(first.unwrap(), true);
    let mut board: Vec<Vec<i32>> = Vec::new();

    lines.next();
    let mut total: usize = 1;
    for line in lines {
        let l = line.unwrap().trim().to_owned();
        if l.len() > 0 {
            board.push(parse_numbers(l, false));
        } else {
            total += 1;
        }
    }

    let mut free: Vec<usize> = (0..total).collect();
    let mut answer: i32 = 0;
    for n in &numbers {
        mark(*n, &mut board);
        let found = check_bingo(&free, &board);
        if found.len() > 0 {
            for b in found {
                let sum = unmarked(b, &board);
                if free.len() == total {
                    println!("Part 1: num = {} * unmarked = {} = {}", n, sum, n*sum);
                }
                free.retain(|&x| x != b);
                answer = n*sum;
                println!("n = {}, b = {}, {}", n, b, answer);
            }
        }
    }
    println!("Part 2: {}", answer);

}