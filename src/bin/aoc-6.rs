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
    for l in board.iter() {
        println!("{}", std::str::from_utf8(l).unwrap());
    }
    // dbg!(board);
    println!("{count}");
    Ok(())
}
