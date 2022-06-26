use aoc2021::utils;

fn main() {
    let nums = utils::read_ints("input/day-1.txt", "");
    let mut prev= &nums[0];
    let mut inc= 0;

    for x in nums.iter() {
      if prev < x {
        inc += 1;
      }
      prev = x;
    }

    println!("Part 1: {}", inc);

    let mut prev: i32 = nums[0..3].iter().sum();
    let mut inc2 = 0;
    for (pos, _) in nums.iter().enumerate() {
      if pos > 0 && pos < nums.len() - 2 {
        let cur: i32 = nums[pos..pos+3].iter().sum();
        if prev < cur {
          inc2 += 1;
        }
        prev = cur;
      }
  }
    println!("Part 2: {}", inc2);
}