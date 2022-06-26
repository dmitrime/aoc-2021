use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Helper from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_int_line(line: &String, sep: &str) -> Vec<i32> {
    if sep != "" {
      line.split(sep).map(|n| n.parse::<i32>().unwrap()).collect()
    } else {
      let num = line.parse::<i32>().unwrap();
      vec![num]
    }
}

pub fn read_ints(path: &str, sep: &str) -> Vec<i32> {
    let lines = read_lines(path).unwrap();
    let mut out: Vec<i32> = Vec::new();
    for line in lines {
      out.extend(read_int_line(&line.unwrap(), sep));
    }
    out
  }