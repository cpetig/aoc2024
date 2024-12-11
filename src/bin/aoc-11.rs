use std::io::{self, stdin, BufRead, BufReader};

fn mutate(pebbles: &mut Vec<usize>) {
    let mut input: Vec<usize> = Vec::new();
    std::mem::swap(pebbles, &mut input);
    for i in input.drain(0..) {
        if i == 0 {
            pebbles.push(1);
        } else {
            let digits = i.ilog10() + 1;
            if digits & 1 == 0 {
                let break_at = usize::pow(10, digits / 2);
                pebbles.push(i / break_at);
                pebbles.push(i % break_at);
            } else {
                pebbles.push(i * 2024);
            }
        }
    }
}

fn pebbles_after(start: usize, count: usize) -> usize {
    if count == 0 {
        1
    } else if start == 0 {
        pebbles_after(1, count - 1)
    } else {
        let digits = start.ilog10() + 1;
        if digits & 1 == 0 {
            let break_at = usize::pow(10, digits / 2);
            pebbles_after(start / break_at, count - 1) + pebbles_after(start % break_at, count - 1)
        } else {
            pebbles_after(2024 * start, count - 1)
        }
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());
    let mut pebbles: Vec<usize> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();
    dbg!(&pebbles);
    for _ in 0..25 {
        mutate(&mut pebbles);
        // dbg!(&pebbles);
    }
    println!("{}", pebbles.len());
    let mut pebble_count=0;
    for i in pebbles.iter() {
        pebble_count+=pebbles_after(*i, 75);
    }
    println!("{pebble_count}");
    Ok(())
}
