use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut score = 0;
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let v = l.split(',').collect::<Vec<_>>();
        let r1 = v[0].split('-').collect::<Vec<_>>();
        let r2 = v[1].split('-').collect::<Vec<_>>();
        let r1s = r1[0].parse::<i32>().unwrap();
        let r1e = r1[1].parse::<i32>().unwrap();
        let r2s = r2[0].parse::<i32>().unwrap();
        let r2e = r2[1].parse::<i32>().unwrap();
        if (r1s >= r2s && r1e <= r2e) || (r2s >= r1s && r2e <= r1e) {
            score += 1;
        }
    }
    println!("{}", score);
}
