// Code from Anders Kaseorg
// https://codegolf.stackexchange.com/a/265342/46901

// modified to use u128 for the count, and a loop in main

use fxhash::FxHashMap;
use std::time::Instant;
use typed_arena::Arena;

fn count_state<'a>(
    target: &[bool],
    distance: u8,
    arena: &'a Arena<u8>,
    cache: &mut FxHashMap<&'a [u8], u128>,
    state: &mut [u8],
) -> u128 {
    let new_state = state;
    let state = arena.alloc_extend(new_state.iter().copied());
    let mut count = 0;
    if state[target.len()] == distance {
        count += 1;
    }
    for symbol in [false, true] {
        let mut z = state[0] + 1;
        new_state[0] = z;
        for (((o, &c), &x), &y) in new_state[1..]
            .iter_mut()
            .zip(target)
            .zip(&state[..state.len() - 1])
            .zip(&state[1..])
        {
            z = (x + u8::from(symbol != c)).min(y.min(z).min(distance) + 1);
            *o = z;
        }

        if new_state.iter().all(|&x| x > distance) {
        } else if let Some(&new_count) = cache.get(new_state) {
            count += new_count;
        } else {
            count += count_state(target, distance, arena, cache, new_state);
        }
    }
    cache.insert(state, count);
    count
}

fn count(target: &[bool], distance: u8) -> u128 {
    let arena = Arena::new();
    let state = arena.alloc_extend(0..=target.len() as u8);
    let mut cache = FxHashMap::default();
    count_state(target, distance, &arena, &mut cache, state)
}

fn main() {
    let start = Instant::now();
    let mut s = Vec::with_capacity(32);
    s.push(false);
    let mut v = 0;
    while s.len() <= 40 { // approx 2 minutes for profiling
        for k in 1u8..(s.len() + 1) as u8 {
            println!("{} {} {}", s.len(), k, count(&s, k));
        }
        s.push([true, true, false, false][v % 4]);
        v += 1;
        println!("{:?}", s)
    }
    println!("{} seconds", start.elapsed().as_secs_f32());
}