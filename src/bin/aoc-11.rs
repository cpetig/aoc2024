use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, stdin, BufRead, BufReader},
};

// fn mutate(pebbles: &mut Vec<usize>) {
//     let mut input: Vec<usize> = Vec::new();
//     std::mem::swap(pebbles, &mut input);
//     for i in input.drain(0..) {
//         if i == 0 {
//             pebbles.push(1);
//         } else {
//             let digits = i.ilog10() + 1;
//             if digits & 1 == 0 {
//                 let break_at = usize::pow(10, digits / 2);
//                 pebbles.push(i / break_at);
//                 pebbles.push(i % break_at);
//             } else {
//                 pebbles.push(i * 2024);
//             }
//         }
//     }
// }

// first is start value, then how many iterations left
type Cache = HashMap<usize, Vec<usize>>;

fn pebbles_after(start: usize, count: usize, cache: &mut Cache) -> usize {
    // dbg!((start,count));
    if count == 0 {
        1
    } else if start == 0 {
        pebbles_after(1, count - 1, cache)
    } else {
        let entry = cache.entry(start);
        if let Entry::Occupied(item) = &entry {
            if let Some(value) = item.get().get(count - 1) {
                if *value > 0 {
                    return *value;
                }
            }
        }
        entry.or_insert(Vec::from_iter(std::iter::repeat_n(0, count)));
        let digits = start.ilog10() + 1;
        let value = if digits & 1 == 0 {
            let break_at = usize::pow(10, digits / 2);
            pebbles_after(start / break_at, count - 1, cache)
                + pebbles_after(start % break_at, count - 1, cache)
        } else {
            pebbles_after(2024 * start, count - 1, cache)
        };
        let entry = cache.entry(start);
        entry.and_modify(|vec| {
            if vec.len() < count {
                vec.extend(std::iter::repeat_n(0, count - vec.len()));
            };
            vec[count - 1] = value
        });
        value
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());
    let pebbles: Vec<usize> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();
    dbg!(&pebbles);
    // for _ in 0..25 {
    //     mutate(&mut pebbles);
    //     // dbg!(&pebbles);
    // }
    // println!("{}", pebbles.len());
    let mut pebble_count = 0;
    let mut cache = Cache::new();
    for i in pebbles.iter() {
        pebble_count += pebbles_after(*i, 75, &mut cache);
    }
    println!("{pebble_count}");
    Ok(())
}
