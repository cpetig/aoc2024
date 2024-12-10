use std::io::{self, stdin, BufRead, BufReader};

fn peaks_from(map: &[String], x: usize, y: usize, next: u8) -> usize {
    dbg!((x,y,next));
    let max_x = map[0].len();
    let max_y = map.len();
    let mut sum = if x > 0 && map[y].as_bytes()[x - 1] == next {
        if next == b'9' {
            1usize
        } else {
            peaks_from(map, x - 1, y, next + 1)
        }
    } else {
        0usize
    };
    sum += if y > 0 && map[y - 1].as_bytes()[x] == next {
        if next == b'9' {
            1usize
        } else {
            peaks_from(map, x, y - 1, next + 1)
        }
    } else {
        0usize
    };
    sum += if y + 1 < max_y && map[y + 1].as_bytes()[x] == next {
        if next == b'9' {
            1usize
        } else {
            peaks_from(map, x, y + 1, next + 1)
        }
    } else {
        0usize
    };
    sum += if x + 1 < max_x && map[y].as_bytes()[x + 1] == next {
        if next == b'9' {
            1usize
        } else {
            peaks_from(map, x + 1, y, next + 1)
        }
    } else {
        0usize
    };
    sum
}

fn main() -> io::Result<()> {
    let mut map = Vec::<String>::new();
    let reader = BufReader::new(stdin());
    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            map.push(input);
        }
    }
    let max_x = map[0].len();
    let max_y = map.len();
    let mut sum = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if map[y].as_bytes()[x] == b'0' {
                sum += peaks_from(&map, x, y, b'1');
            }
        }
    }
    println!("{sum}");
    Ok(())
}
