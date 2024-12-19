use std::io::{self, stdin, BufRead, BufReader};

use itertools::Itertools;

fn get_combo(value: u8, a: usize, b: usize, c: usize) -> usize {
    match value {
        0..=3 => value as usize,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    // let mut count: usize = 0;
    // let mut count2: usize = 0;
    let mut register_a: usize = 0;
    let mut register_b: usize = 0;
    let mut register_c: usize = 0;
    let mut output: Vec<usize> = Vec::new();
    let mut program: Vec<u8> = Vec::new();
    let mut program_counter: usize = 0;

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            let mut elem = input.split_whitespace();
            let typ = elem.next().unwrap();
            if typ == "Register" {
                let which = elem.next().unwrap();
                let value: usize = elem.next().unwrap().parse().unwrap();
                if which == "A:" {
                    register_a = value;
                } else if which == "B:" {
                    register_b = value;
                } else if which == "C:" {
                    register_c = value;
                } else {
                    unreachable!();
                }
            } else if typ == "Program:" {
                let code = input[9..].split(',');
                for i in code {
                    program.push(i.parse().unwrap());
                }
            } else {
                unreachable!();
            }
        }
    }
    // dbg!((&register_a, &register_b, &register_c, &program));
    loop {
        if program_counter >= program.len() {
            break;
        }
        let operand = program[program_counter + 1];
        match program[program_counter] {
            0 => {
                //adv
                register_a >>= get_combo(operand, register_a, register_b, register_c);
            }
            1 =>
            //bxl
            {
                register_b ^= operand as usize
            }
            2 =>
            //bst
            {
                register_b = get_combo(operand, register_a, register_b, register_c) & 7
            }
            3 => {
                //jnz
                if register_a != 0 {
                    program_counter = operand as usize;
                    continue;
                }
            }
            4 =>
            //bxc
            {
                register_b ^= register_c
            }
            5 => {
                // out
                output.push(get_combo(operand, register_a, register_b, register_c) & 7);
            }
            6 => {
                //bdv
                register_b = register_a >> get_combo(operand, register_a, register_b, register_c);
            }
            7 => {
                //cdv
                register_c = register_a >> get_combo(operand, register_a, register_b, register_c);
            }

            _ => unreachable!(),
        }
        program_counter += 2;
        // println!(
        //     "Step {} {} {} {}",
        //     register_a, register_b, register_c, program_counter
        // );
    }
    // dbg!(&output);
    let output2: Vec<u8> = output
        .iter()
        .map(|n| (*n as u8) + b'0')
        .intersperse(b',')
        .collect();
    println!("{}", &String::from_utf8(output2).unwrap());
    // println!("{count} {count2}");
    Ok(())
}
