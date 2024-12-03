use std::io::{self, stdin, BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(stdin());

    let mut vec_a: Vec<usize> = std::vec::Vec::new();
    let mut vec_b: Vec<usize> = std::vec::Vec::new();
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        // dbg!(&line);
        if len == 0 {
            // dbg!();
            break;
        }
        let mut numbers = line.split_whitespace();
        let (num_a, num_b) = (
            numbers.next().unwrap().parse().unwrap(),
            numbers.next().unwrap().parse().unwrap(),
        );
        vec_a.push(num_a);
        vec_b.push(num_b);
    }
    // dbg!(&vec_a);
    vec_a.sort();
    vec_b.sort();
    let mut distance = 0;
    for (a, b) in vec_a.iter().zip(vec_b.iter()) {
        distance += a.abs_diff(*b);
    }
    println!("Distance {distance}");

    let similarity: usize = vec_a.iter().fold(0, |acc, &a| {
        acc + a * vec_b.iter().filter(|&&b| a == b).count()
    });
    println!("Similarity {similarity}");
    Ok(())
}
