use std::io::{self, stdin, BufRead, BufReader};

fn valid(numbers: &[usize]) -> bool {
    let ascending = numbers[1] > numbers[0];
    numbers
        .iter()
        .fold((true, None), |acc: (bool, Option<usize>), elem| {
            if let Some(old) = acc.1 {
                let valid_range = if ascending {
                    (old + 1)..=(old + 3)
                } else {
                    (old.saturating_sub(3))..=(old.saturating_sub(1))
                };
                // dbg!(&valid_range);
                (acc.0 && valid_range.contains(elem), Some(*elem))
            } else {
                (true, Some(*elem))
            }
        })
        .0
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());
    let mut ok_lines = 0;
    let mut ok_dampened = 0;

    for str in reader.lines().map_while(Result::ok) {
        let numbers: Vec<usize> = str
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();
        if valid(&numbers) {
            ok_lines += 1;
            ok_dampened += 1;
        } else {
            for i in 0..numbers.len() {
                let mut without_n = numbers.clone();
                without_n.remove(i);
                if valid(&without_n) {
                    ok_dampened += 1;
                    break;
                }
            }
        }
    }
    println!("{ok_lines}");
    println!("{ok_dampened}");
    Ok(())
}
