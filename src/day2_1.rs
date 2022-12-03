use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut score = 0;

    let map1 = HashMap::from([
        ("A X", 3),
        ("A Y", 6),
        ("A Z", 0),
        ("B X", 0),
        ("B Y", 3),
        ("B Z", 6),
        ("C X", 6),
        ("C Y", 0),
        ("C Z", 3),
    ]);

    let map2 = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let m = l.split_whitespace().collect::<Vec<_>>();
        let s1 = map1[&*l];
        let s2 = map2[&m[1]];
        score += s1 + s2;
    }
    println!("{}", score);
}
