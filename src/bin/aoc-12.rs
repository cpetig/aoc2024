use std::io::{self, stdin, BufRead, BufReader};

const ALREADY_SEEN: u8 = 0x80;

pub fn garden(board: &mut [Vec<u8>], x: usize, y: usize) -> (usize, usize, usize) {
    let mut area = 1;
    let mut perimeter = 0;
    let mut discount = 0;
    let center = board[y][x];
    board[y][x] |= ALREADY_SEEN;
    let same = |board: &[Vec<u8>], x, y, cmp| -> bool {
        board.get(y).map_or(false, |v: &Vec<u8>| {
            v.get(x).map_or(false, |val| val & !ALREADY_SEEN == cmp)
        })
    };
    // only count the top leftmost of each perimeter line
    // top
    if !same(board, x, y.wrapping_sub(1), center) {
        // don't count if perimeter continues to the left
        if !(same(board, x.wrapping_sub(1), y, center)
            && !same(board, x.wrapping_sub(1), y.wrapping_sub(1), center))
        {
            discount += 1;
        }
        perimeter += 1;
    } else {
        if board[y - 1][x] == center {
            let (subareara, subperimeter, subdiscount) = garden(board, x, y - 1);
            area += subareara;
            perimeter += subperimeter;
            discount += subdiscount;
        }
    }
    // bottom
    if !same(board, x, y + 1, center) {
        // don't count if perimeter continues to the left
        if !(same(board, x.wrapping_sub(1), y, center)
            && !same(board, x.wrapping_sub(1), y + 1, center))
        {
            discount += 1;
        }
        perimeter += 1;
    } else {
        if board[y + 1][x] == center {
            let (subareara, subperimeter, subdiscount) = garden(board, x, y + 1);
            area += subareara;
            perimeter += subperimeter;
            discount += subdiscount;
        }
    }
    // left
    if !same(board, x.wrapping_sub(1), y, center) {
        // don't count if perimeter continues to the top
        if !(same(board, x, y.wrapping_sub(1), center)
            && !same(board, x.wrapping_sub(1), y.wrapping_sub(1), center))
        {
            discount += 1;
        }
        perimeter += 1;
    } else {
        if board[y][x - 1] == center {
            let (subareara, subperimeter, subdiscount) = garden(board, x - 1, y);
            area += subareara;
            perimeter += subperimeter;
            discount += subdiscount;
        }
    }
    // right
    if !same(board, x + 1, y, center) {
        // don't count if perimeter continues to the top
        if !(same(board, x, y.wrapping_sub(1), center)
            && !same(board, x + 1, y.wrapping_sub(1), center))
        {
            discount += 1;
        }
        perimeter += 1;
    } else {
        if board[y][x + 1] == center {
            let (subareara, subperimeter, subdiscount) = garden(board, x + 1, y);
            area += subareara;
            perimeter += subperimeter;
            discount += subdiscount;
        }
    }
    (area, perimeter, discount)
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut board: Vec<Vec<u8>> = Vec::new();
    let mut count: usize = 0;
    let mut count2: usize = 0;

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            let line = input.as_bytes();
            board.push(Vec::from(line));
        }
    }
    let max_x = board[0].len();
    let max_y = board.len();
    for y in 0..max_y {
        for x in 0..max_x {
            if board[y][x] < 128 {
                let (area, perimeter, discount) = garden(&mut board, x, y);
                count += area * perimeter;
                count2 += area * discount;
            }
        }
    }
    println!("{count} {count2}");
    Ok(())
}
