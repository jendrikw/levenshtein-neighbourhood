#![warn(clippy::all)]

use std::collections::VecDeque;
use std::collections::hash_map::Entry;
use std::time::Instant;

use nohash_hasher::{IntMap, IntSet};

fn count_fixed_distance(s: &str, k: u32) -> usize {
    let n = i32::from_str_radix(&format!("1{s}"), 2).unwrap();
    let mut distances = IntMap::with_capacity_and_hasher(1 << (s.len() + k as usize), Default::default());
    distances.insert(n, 0);
    let mut queue = VecDeque::from([(n, s.len())]);
    // capacity is just experimentation
    let mut valid = IntSet::with_capacity_and_hasher(1 << (k + 1), Default::default());
    while let Some((current, bit_count)) = queue.pop_front() {
        let next_dist = distances.get(&current).unwrap() + 1;
        let mut pow = 0;
        for e in 0..bit_count {
            let left = current >> e;
            let left_1 = left << 1;
            pow = 1 << e;
            let right = current % pow;
            let bp1 = bit_count + 1;
            for (next, next_bit_count) in [
                ((left >> 1 << e) | right, bit_count - 1),
                (current ^ pow, bit_count),
                ((left_1 << e) | right, bp1),
                (((left_1 | 1) << e) | right, bp1)
            ] {
                match distances.entry(next) {
                    Entry::Vacant(ve) => {
                        ve.insert(next_dist);
                    }
                    Entry::Occupied(_) => continue,
                }
                if next_dist < k {
                    queue.push_back((next, next_bit_count));
                } else {
                    valid.insert(next);
                }
            }
        }

        for next in [pow << 2 | current, 3 << bit_count ^ current] {
            match distances.entry(next) {
                Entry::Vacant(ve) => {
                    ve.insert(next_dist);
                }
                Entry::Occupied(_) => continue,
            }
            if next_dist < k {
                queue.push_back((next, bit_count + 1));
            } else {
                valid.insert(next);
            }
        }
    }
    valid.len()
}

fn main() {
    let start = Instant::now();
    let mut s = String::with_capacity(32);
    s.push('0');
    let mut v = 0;
    while s.len() <= 12 { // approx 2 minutes for profiling
        for k in 1..(s.len() + 1) {
            println!("{} {} {}", s.len(), k, count_fixed_distance(&s, k as u32));
        }
        s.push(['1', '1', '0', '0'][v % 4]);
        v += 1;
        println!("{}", s)
    }
    println!("{} seconds", start.elapsed().as_secs_f32());
}
