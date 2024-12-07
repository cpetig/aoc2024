// use itertools::Itertools;
use std::io::{self, stdin, BufRead, BufReader};

fn concat(a: usize, b: usize) -> usize {
    assert!(b != 0);
    let log = b.ilog10() + 1;
    a * usize::pow(10, log) + b
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut sum: usize = 0;
    let mut sum2: usize = 0;

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            if let Some((value, terms)) = input.split_once(":") {
                let value: usize = value.parse().unwrap();
                let terms: Vec<usize> = terms
                    .split_whitespace()
                    .map(|t| t.parse::<usize>().unwrap())
                    .collect();
                assert!(terms.len() <= 32);
                let combinations = 1 << terms.len();
                for mut i in 0..combinations {
                    let calc: Option<usize> = terms.iter().copied().reduce(|a, b| {
                        let res = if i % 2 == 0 { a + b } else { a * b };
                        i = i / 2;
                        res
                    });
                    // dbg!(&calc);
                    if calc == Some(value) {
                        sum += value;
                        break;
                    }
                }
                let combinations2 = usize::pow(3, terms.len() as u32);
                for mut i in 0..combinations2 {
                    let calc: Option<usize> = terms.iter().copied().reduce(|a, b| {
                        let res = match i % 3 {
                            0 => a + b,
                            1 => a * b,
                            2 => concat(a, b),
                            _ => unreachable!(),
                        };
                        i = i / 3;
                        res
                    });
                    // dbg!(&calc);
                    if calc == Some(value) {
                        sum2 += value;
                        break;
                    }
                }
            }
        }
    }
    println!("{sum} {sum2}");
    Ok(())
}
