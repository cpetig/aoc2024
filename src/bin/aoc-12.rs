use std::{
    collections::HashMap,
    io::{self, stdin, BufRead, BufReader},
};

const ALREADY_SEEN: u8 = 0x80;

pub fn garden(board: &mut [Vec<u8>], x: usize, y: usize) -> (usize, usize) {
    let max_x = board[0].len();
    let max_y = board.len();
    let mut area = 1;
    let mut perimeter = 0;
    let center = board[y][x];
    board[y][x] |= ALREADY_SEEN;
    if y == 0 || (board[y - 1][x] & !ALREADY_SEEN) != center {
        perimeter += 1;
    } else {
        if board[y - 1][x] == center {
            let (subareara, subperimeter) = garden(board, x, y - 1);
            area += subareara;
            perimeter += subperimeter;
        }
    }
    if y + 1 == max_y || (board[y + 1][x] & !ALREADY_SEEN) != center {
        perimeter += 1;
    } else {
        if board[y + 1][x] == center {
            let (subareara, subperimeter) = garden(board, x, y + 1);
            area += subareara;
            perimeter += subperimeter;
        }
    }
    if x == 0 || (board[y][x - 1] & !ALREADY_SEEN) != center {
        perimeter += 1;
    } else {
        if board[y][x - 1] == center {
            let (subareara, subperimeter) = garden(board, x - 1, y);
            area += subareara;
            perimeter += subperimeter;
        }
    }
    if x + 1 == max_x || (board[y][x + 1] & !ALREADY_SEEN) != center {
        perimeter += 1;
    } else {
        if board[y][x + 1] == center {
            let (subareara, subperimeter) = garden(board, x + 1, y);
            area += subareara;
            perimeter += subperimeter;
        }
    }
    (area, perimeter)
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut board: Vec<Vec<u8>> = Vec::new();
    let mut count: usize = 0;

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            let line = input.as_bytes();
            board.push(Vec::from(line));
        }
    }
    // dbg!(&antennas);
    let max_x = board[0].len();
    let max_y = board.len();
    for y in 0..max_y {
        for x in 0..max_x {
            if board[y][x] < 128 {
                let (area, perimeter) = garden(&mut board, x, y);
                count += area * perimeter;
            }
        }
    }
    println!("{count}");
    Ok(())
}
