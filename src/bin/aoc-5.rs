use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{self, stdin, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut ordering: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut sum = 0;
    let mut sum_corrected = 0;

    for input in reader.lines().map_while(Result::ok) {
        if input.contains("|") {
            let mut parts = input.split("|");
            let a = parts.next().unwrap().parse().unwrap();
            let b = parts.next().unwrap().parse().unwrap();
            ordering.entry(a).or_insert(HashSet::new()).insert(b);
        } else if input.contains(",") {
            let mut update: Vec<usize> = input.split(",").map(|i| i.parse().unwrap()).collect();
            let mut ok = true;
            for i in 1..update.len() {
                let pos = update.len() - i;
                let second = update[pos];
                if let Some(map) = ordering.get(&second) {
                    // dbg!(update.split_at(pos), second);
                    if update.split_at(pos).0.iter().any(|e| map.contains(e)) {
                        // dbg!(pos, second);
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                sum += update[update.len() / 2];
            } else {
                update.sort_by(|a, b| {
                    // dbg!((a, b));
                    if ordering.get(a).map_or(false, |o| o.contains(b)) {
                        Ordering::Less
                    } else if ordering.get(b).map_or(false, |o| o.contains(a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                sum_corrected += update[update.len() / 2];
            }
        }
    }
    // dbg!(&ordering);
    println!("{sum} {sum_corrected}");
    Ok(())
}
