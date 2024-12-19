use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, stdin, BufRead, BufReader},
};

fn possible(input: &str, towels: &Vec<String>, known: &mut HashMap<usize, usize>) -> usize {
    if input.is_empty() {
        return 1;
    }
    let k = known.entry(input.len());
    if let Entry::Occupied(value) = k {
        return *value.get();
    }
    let mut result = 0;
    for towel in towels.iter() {
        if towel.len() <= input.len() {
            if &input[..towel.len()] == towel.as_str() {
                // println!("{} {} {}", input.len(), input, towel);
                result += possible(&input[towel.len()..], towels, known);
            }
        }
    }
    known.insert(input.len(), result);
    result
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut count: usize = 0;
    let mut count2: usize = 0;

    let mut lines = reader.lines().map_while(Result::ok);

    let towelstring = lines.next().unwrap();
    let mut towels: Vec<String> = Vec::new();
    for towel in towelstring.split(&[',', ' ']) {
        if !towel.is_empty() {
            towels.push(towel.into());
        }
    }
    dbg!(&towels);

    for input in lines {
        if !input.is_empty() {
            dbg!(&input);
            let mut known: HashMap<usize, usize> = HashMap::new();
            let possibilities = possible(&input, &towels, &mut known);
            if possibilities > 0 {
                count += 1;
            }
            count2 += possibilities;
        }
    }
    println!("{count} {count2}");
    Ok(())
}
