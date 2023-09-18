/*
from collections import deque

def countFixedDistance(s: str, k: int) -> int:
    n = int('1' + s, 2)
    distances: dict[int, int] = {n: 0}
    queue = deque(((n, len(s)),))
    valid: set[int] = set()
    while len(queue) > 0:
        current, bit_count = queue.popleft()
        next_dist = distances[current] + 1
        next_bit_count = bit_count - 1
        for next_gen in (
            ((current >> e + 1 << e) + current % (1 << e) for e in range(bit_count)),
            (current ^ (1 << e) for e in range(bit_count)),
            (((current >> e << 1 | b) << e) + current % (1 << e) for b in (0, 1) for e in range(bit_count + 1)),
        ):
          for next in next_gen:
              if next in distances: continue
              distances[next] = next_dist
              if next_dist < k:
                  queue.append((next, next_bit_count))
              else:
                  valid.add(next)
          next_bit_count += 1
    return len(valid)
 */
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::collections::btree_map::Entry;
use std::time::Instant;

use fasthash::sea::Hash64;

fn count_fixed_distance(s: &str, k: u32) -> usize {
    let n = i32::from_str_radix(&format!("1{s}"), 2).unwrap();
    let mut distances = BTreeMap::from([(n, 0)]);
    let mut queue = VecDeque::from([(n, s.len())]);
    let mut valid = BTreeSet::new();
    while let Some((current, bit_count)) = queue.pop_front() {
        let next_dist = distances.get(&current).unwrap() + 1;
        let mut next_bit_count = bit_count - 1;
        for next_gen in [
            Box::new((0..bit_count).map(|e| (current >> (e + 1) << e) + current % (1 << e))) as Box<dyn Iterator<Item=i32>>,
            Box::new((0..bit_count).map(|e| current ^ (1 << e))),
            Box::new((0..bit_count).map(|e| (current >> e << 1 << e) + current % (1 << e))),
            Box::new((0..bit_count).map(|e| ((current >> e << 1 | 1) << e) + current % (1 << e))),
        ] {
            for next in next_gen {
                // if distances.contains_key(&next) {
                //     continue;
                // }
                // distances.insert(next, next_dist);
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
            next_bit_count += 1;
        }
    }
    valid.len()
}

fn main() {
    let start = Instant::now();
    let mut s = String::with_capacity(32);
    s.push('0');
    let mut v = 0;
    while s.len() <= 11 { // approx 1 minute for profiling
        for k in 1..(s.len() + 1) {
            println!("{} {} {}", s.len(), k, count_fixed_distance(&s, k as u32));
        }
        s.push(['1', '1', '0', '0'][v % 4]);
        v += 1;
        println!("{}", s)
    }
    println!("{} seconds", start.elapsed().as_secs_f32());
}
