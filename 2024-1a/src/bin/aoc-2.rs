use std::io::{self, stdin, BufRead, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());
    let mut ok_lines = 0;
    let mut ok_dampened = 0;

    for l in reader.lines() {
        if let Ok(str) = l {
            let numbers: Vec<usize> = str
                .split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect();
            let ascending = numbers[1] > numbers[0];
            let valid = numbers
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
                .0;
            // dbg!((&numbers, valid));
            if valid {
                ok_lines += 1;
            }

            let ascending_count = if numbers[1] > numbers[0] { 1 } else { 0 }
                + if numbers[2] > numbers[1] { 1 } else { 0 }
                + if numbers[3] > numbers[2] { 1 } else { 0 };
            let ascending = ascending_count >= 2;
            let valid_dampened = numbers
                .iter()
                .fold(
                    (true, true, None),
                    |(valid, skip, last): (bool, bool, Option<usize>), &elem| {
                        if let Some(old) = last {
                            let valid_range = if ascending {
                                (old + 1)..=(old + 3)
                            } else {
                                (old.saturating_sub(3))..=(old.saturating_sub(1))
                            };
                            // dbg!(&valid_range);
                            if valid_range.contains(&elem) {
                                (valid, skip, Some(elem))
                            } else if skip {
                                (valid, false, Some(old))
                            } else {
                                (false, false, Some(elem))
                            }
                        } else {
                            (true, true, Some(elem))
                        }
                    },
                )
                .0;
            // dbg!((&numbers, valid));
            if valid_dampened {
                ok_dampened += 1;
            }
            if valid != valid_dampened {
                dbg!(&numbers);
            }
        }
    }
    println!("{ok_lines}");
    println!("{ok_dampened}");
    Ok(())
}
