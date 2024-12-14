use std::io::{self, stdin, BufRead, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut count: [usize; 4] = [0, 0, 0, 0];
    const SIZEX: isize = 101; // 11 101
    const SIZEY: isize = 103; // 7 103
    const TIME: isize = 100;
    const SIZEX_HALF: usize = SIZEX as usize / 2;
    const SIZEY_HALF: usize = SIZEY as usize / 2;

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            let mut elem = input.split_whitespace();
            let pos = elem.next().unwrap();
            let velocity = elem.next().unwrap();
            assert!(pos.chars().next() == Some('p'));
            assert!(pos.chars().nth(1) == Some('='));
            let mut positer = pos[2..].split(',');
            let (px, py): (usize, usize) = (
                positer.next().unwrap().parse().unwrap(),
                positer.next().unwrap().parse().unwrap(),
            );
            assert!(velocity.chars().next() == Some('v'));
            assert!(velocity.chars().nth(1) == Some('='));
            let mut veliter = velocity[2..].split(',');
            let (vx, vy): (isize, isize) = (
                veliter.next().unwrap().parse().unwrap(),
                veliter.next().unwrap().parse().unwrap(),
            );
            let mut finalx = (px as isize + TIME * vx) % SIZEX;
            let mut finaly = (py as isize + TIME * vy) % SIZEY;
            if finalx < 0 {
                finalx += SIZEX;
            }
            if finaly < 0 {
                finaly += SIZEY;
            }
            // dbg!((finalx, finaly));
            let (finalx, finaly) = (finalx as usize, finaly as usize);
            if finalx != SIZEX_HALF && finaly != SIZEY_HALF {
                let top = finaly < SIZEY_HALF;
                let left = finalx < SIZEX_HALF;
                let index: usize = if top { 0 } else { 2 } + if left { 0 } else { 1 };
                count[index] += 1;
            }
        }
    }
    dbg!(&count);
    println!("{}", count[0] * count[1] * count[2] * count[3]);
    Ok(())
}
