use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut curr_max = 0;
    let mut curr_elf = 0;
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.len() > 0 {
            let my_int = l.parse::<i32>().unwrap();
            curr_elf += my_int
        } else {
            if curr_elf > curr_max {
                curr_max = curr_elf;
            }
            curr_elf = 0;
        }
    }
    println!("{}", curr_max);
}
