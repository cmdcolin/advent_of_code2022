use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut vec = Vec::new();
    let mut curr_elf = 0;
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.len() > 0 {
            let my_int = l.parse::<i32>().unwrap();
            curr_elf += my_int
        } else {
            vec.push(curr_elf);
            curr_elf = 0;
        }
    }
    vec.sort();
    vec.reverse();

    println!("{}", vec[0] + vec[1] + vec[2]);
}
