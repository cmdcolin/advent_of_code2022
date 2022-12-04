use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut score = 0;
    for line in stdin.lock().lines() {
        let mut set = HashSet::new();
        let l = line.unwrap();
        let s1 = &l[..l.len() / 2];
        let s2 = &l[l.len() / 2..];
        for i in s1.chars() {
            set.insert(i);
        }
        let mut l = '\0';
        for i in s2.chars() {
            if set.get(&i) != None {
                l = i;
                break;
            }
        }
        let k = l as u32;
        let lower_a = 'a' as u32;
        let upper_a = 'A' as u32;
        let val = if l.is_lowercase() {
            k - lower_a + 1
        } else {
            k - upper_a + 27
        };
        score += val;
    }
    println!("{}", score);
}
