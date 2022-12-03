use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut score = 0;

    let map0 = HashMap::from([
        ("A X", "Z"),
        ("A Y", "X"),
        ("A Z", "Y"),
        ("B X", "X"),
        ("B Y", "Y"),
        ("B Z", "Z"),
        ("C X", "Y"),
        ("C Y", "Z"),
        ("C Z", "X"),
    ]);

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
        let m1 = l.split_whitespace().collect::<Vec<_>>();
        let choice = map0[&*l];
        let panel = format!("{} {}", m1[0], choice);
        let s1 = map1[&*panel];
        let s2 = map2[&choice];
        score += s1 + s2;
    }
    println!("{}", score);
}
