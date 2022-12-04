use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut score = 0;
    let mut group_counter = 0;
    let mut sets = [HashSet::new(), HashSet::new(), HashSet::new()];
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        for i in l.chars() {
            sets[group_counter].insert(i);
        }
        group_counter = group_counter + 1;

        if group_counter == 3 {
            {
                let (intersection, others) = sets.split_at_mut(1);
                let intersection = &mut intersection[0];
                for other in others {
                    intersection.retain(|e| other.contains(e));
                }
                let v: Vec<_> = intersection.iter().collect();
                let k = *v[0] as u32;
                let lower_a = 'a' as u32;
                let upper_a = 'A' as u32;
                let val = if v[0].is_lowercase() {
                    k - lower_a + 1
                } else {
                    k - upper_a + 27
                };
                score += val;
            }
            group_counter = 0;
            sets = [HashSet::new(), HashSet::new(), HashSet::new()];
        }
    }
    println!("{}", score);
}
