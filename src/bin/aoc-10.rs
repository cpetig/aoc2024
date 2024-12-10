use std::{
    collections::HashSet,
    io::{self, stdin, BufRead, BufReader},
};

fn rating(map: &[String], x: usize, y: usize, next: u8) -> usize {
    // dbg!((x,y,next));
    let max_x = map[0].len();
    let max_y = map.len();
    let mut sum = if x > 0 && map[y].as_bytes()[x - 1] == next {
        if next == b'9' {
            1usize
        } else {
            rating(map, x - 1, y, next + 1)
        }
    } else {
        0usize
    };
    sum += if y > 0 && map[y - 1].as_bytes()[x] == next {
        if next == b'9' {
            1usize
        } else {
            rating(map, x, y - 1, next + 1)
        }
    } else {
        0usize
    };
    sum += if y + 1 < max_y && map[y + 1].as_bytes()[x] == next {
        if next == b'9' {
            1usize
        } else {
            rating(map, x, y + 1, next + 1)
        }
    } else {
        0usize
    };
    sum += if x + 1 < max_x && map[y].as_bytes()[x + 1] == next {
        if next == b'9' {
            1usize
        } else {
            rating(map, x + 1, y, next + 1)
        }
    } else {
        0usize
    };
    sum
}

fn peaks_from(map: &[String], x: usize, y: usize, next: u8, peaks: &mut HashSet<(usize, usize)>) {
    // dbg!((x, y, next));
    let max_x = map[0].len();
    let max_y = map.len();
    if x > 0 && map[y].as_bytes()[x - 1] == next {
        if next == b'9' {
            peaks.insert((x - 1, y));
        } else {
            peaks_from(map, x - 1, y, next + 1, peaks);
        }
    }
    if y > 0 && map[y - 1].as_bytes()[x] == next {
        if next == b'9' {
            peaks.insert((x, y-1));
        } else {
            peaks_from(map, x, y - 1, next + 1, peaks);
        }
    }
    if y + 1 < max_y && map[y + 1].as_bytes()[x] == next {
        if next == b'9' {
            peaks.insert((x, y+1));
        } else {
            peaks_from(map, x, y + 1, next + 1, peaks);
        }
    }
    if x + 1 < max_x && map[y].as_bytes()[x + 1] == next {
        if next == b'9' {
            peaks.insert((x + 1, y));
        } else {
            peaks_from(map, x + 1, y, next + 1, peaks);
        }
    }
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
    let mut rating_sum = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if map[y].as_bytes()[x] == b'0' {
                let mut peaks = HashSet::new();
                peaks_from(&map, x, y, b'1', &mut peaks);
                sum += peaks.len();
                rating_sum += rating(&map, x,y, b'1');
            }
        }
    }
    println!("{sum} {rating_sum}");
    Ok(())
}
