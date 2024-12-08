use std::{
    collections::HashMap,
    io::{self, stdin, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut board: Vec<Vec<u8>> = Vec::new();
    let mut count: usize = 0;
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let valid_chars = [b'0'..=b'9', b'a'..=b'z', b'A'..=b'Z'];

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            let line = input.as_bytes();
            for (x, ch) in line.iter().enumerate() {
                if valid_chars[0].contains(ch)
                    || valid_chars[1].contains(ch)
                    || valid_chars[2].contains(ch)
                {
                    antennas
                        .entry(*ch)
                        .or_insert(Vec::new())
                        .push((x, board.len()));
                }
            }
            board.push(Vec::from(line));
        }
    }
    dbg!(&antennas);
    let max_x = board[0].len() as isize;
    let max_y = board.len() as isize;
    for (_ch, a) in antennas.iter() {
        for pos1 in a.iter() {
            for pos2 in a.iter() {
                if pos1 != pos2 {
                    let candidate = (
                        2 * pos2.0 as isize - pos1.0 as isize,
                        2 * pos2.1 as isize - pos1.1 as isize,
                    );
                    if candidate.0 >= 0
                        && candidate.0 < max_x
                        && candidate.1 >= 0
                        && candidate.1 < max_y
                    {
                        let candidate = (candidate.0 as usize, candidate.1 as usize);
                        if board[candidate.1][candidate.0] != b'#' {
                            board[candidate.1][candidate.0] = b'#';
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    let mut count2: usize = 0;
    for (_ch, a) in antennas.iter() {
        for pos1 in a.iter() {
            for pos2 in a.iter() {
                if pos1 != pos2 {
                    let pos1 = (pos1.0 as isize, pos1.1 as isize);
                    let pos2 = (pos2.0 as isize, pos2.1 as isize);
                    for n in 0..isize::MAX {
                        let candidate = (
                            pos1.0 + n * (pos2.0 - pos1.0),
                            pos1.1 + n * (pos2.1 - pos1.1),
                        );
                        if candidate.0 >= 0
                            && candidate.0 < max_x
                            && candidate.1 >= 0
                            && candidate.1 < max_y
                        {
                            let candidate = (candidate.0 as usize, candidate.1 as usize);
                            if board[candidate.1][candidate.0] != b'#' {
                                board[candidate.1][candidate.0] = b'#';
                                count2 += 1;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{count} {}", count + count2);
    Ok(())
}
