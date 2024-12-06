use std::io::{self, stdin, BufRead, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut board: Vec<Vec<u8>> = Vec::new();
    let mut position: (usize, usize) = (0, 0);
    let mut count: usize = 0;
    let mut direction: (isize, isize) = (0, -1);

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            let line = input.as_bytes();
            if let Some(x) = line.iter().position(|&c| c == b'^') {
                position = (x, board.len());
            }
            board.push(Vec::from(line));
        }
    }
    let initial_position = position;
    assert!(board[position.1][position.0] == b'^');
    board[position.1][position.0] = b'X';
    count += 1;
    loop {
        let newpos = (
            position.0 as isize + direction.0,
            position.1 as isize + direction.1,
        );
        if newpos.0 < 0
            || newpos.0 as usize >= board[0].len()
            || newpos.1 < 0
            || newpos.1 as usize >= board.len()
        {
            break;
        }
        let newpos = (newpos.0 as usize, newpos.1 as usize);
        match board[newpos.1][newpos.0] {
            b'.' => {
                board[newpos.1][newpos.0] = b'X';
                count += 1;
                position = newpos;
            }
            b'#' => {
                // rotate right
                direction = (-direction.1, direction.0);
            }
            b'X' => position = newpos,
            _ => unreachable!(),
        }
    }
    // for l in board.iter() {
    //     println!("{}", std::str::from_utf8(l).unwrap());
    // }
    // dbg!(board);
    println!("{count}");
    let mut new_obstacles = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == b'X' && (x, y) != initial_position {
                // potential loop
                let mut testboard = board.clone();
                testboard[y][x] = b'#';
                position = initial_position;
                direction = (0, -1);
                let is_loop = loop {
                    let newpos = (
                        position.0 as isize + direction.0,
                        position.1 as isize + direction.1,
                    );
                    if newpos.0 < 0
                        || newpos.0 as usize >= board[0].len()
                        || newpos.1 < 0
                        || newpos.1 as usize >= board.len()
                    {
                        // for l in testboard.iter() {
                        //     println!("{}", std::str::from_utf8(l).unwrap());
                        // }
                        break false;
                    }
                    let newpos = (newpos.0 as usize, newpos.1 as usize);
                    match testboard[newpos.1][newpos.0] {
                        b'.' | b'X' | b'u' | b'd' | b'l' | b'r' => {
                            const DIRECTION: &[u8] = b"ul*rd";
                            let dir_code = DIRECTION[(direction.0 + direction.1 * 2 + 2) as usize];
                            if testboard[newpos.1][newpos.0] == dir_code {
                                // dbg!(dir_code);
                                break true;
                            }
                            testboard[newpos.1][newpos.0] = dir_code;
                            position = newpos;
                        }
                        b'#' => {
                            // rotate right
                            direction = (-direction.1, direction.0);
                        }
                        _ => unreachable!(),
                    }
                };
                if is_loop {
                    // for l in testboard.iter() {
                    //     println!("{}", std::str::from_utf8(l).unwrap());
                    // }
                    new_obstacles += 1;
                }
            }
        }
    }
    println!("{new_obstacles}");
    Ok(())
}
