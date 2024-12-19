use std::io::{self, stdin, BufRead, BufReader};

fn possible(input: &str, towels: &Vec<String>) -> bool {
    if input.is_empty() { return true; }
    for (n, towel) in towels.iter().enumerate() {
        if towel.len()<=input.len() {
            if &input[..towel.len()]==towel.as_str() {
                println!("{} {} {} {}",input.len(), n, input, towel);
                if possible(&input[towel.len()..], towels) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut count: usize = 0;

    let mut lines = reader.lines().map_while(Result::ok);

    let towelstring = lines.next().unwrap();
    let mut towels : Vec<String> = Vec::new();
    for towel in towelstring.split(&[',', ' ']) {
        if !towel.is_empty() {
            towels.push(towel.into());
        }
    }
    dbg!(&towels);

    for input in lines {
        if !input.is_empty() {
            dbg!(&input);
            if possible(&input, &towels) {
                count+=1;
            }
        }
    }
    println!("{count}");
    Ok(())
}
