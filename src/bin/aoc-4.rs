use std::io::{self, stdin, BufRead, BufReader};

const PATTERN: &[u8] = b"XMAS";

fn test(
    board: &[String],
    pattern: &[u8],
    mut x: usize,
    mut y: usize,
    dx: isize,
    dy: isize,
) -> bool {
    for p in pattern {
        if board[y].as_bytes()[x] != *p {
            return false;
        }
        y = (y as isize + dy) as usize;
        x = (x as isize + dx) as usize;
    }
    true
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut board: Vec<String> = Vec::new();
    let mut count = 0;

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            board.push(input);
        }
    }
    // dbg!(&board);
    for y in 0..=board.len() - 4 {
        for x in 0..=board[0].len() - 4 {
            if test(&board, PATTERN, x, y, 1, 0) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x, y, 0, 1) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x, y, 1, 1) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x + 3, y, -1, 1) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x + 3, y, -1, 0) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x, y + 3, 0, -1) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x + 3, y + 3, -1, -1) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x, y + 3, 1, -1) {
                // dbg!((x, y));
                count += 1;
            }
        }
        for x in board[0].len() - 3..=board[0].len() - 1 {
            if test(&board, PATTERN, x, y, 0, 1) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x, y + 3, 0, -1) {
                // dbg!((x, y));
                count += 1;
            }
        }
    }
    for y in board.len() - 3..=board.len() - 1 {
        for x in 0..=board[0].len() - 4 {
            if test(&board, PATTERN, x, y, 1, 0) {
                // dbg!((x, y));
                count += 1;
            }
            if test(&board, PATTERN, x+3, y, -1, 0) {
                // dbg!((x, y));
                count += 1;
            }
        }
    }
    println!("{count}");
    Ok(())
}