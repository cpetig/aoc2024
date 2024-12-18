use core::str;
use std::{
    array,
    ffi::OsString,
    io::{self, stdin, BufRead, BufReader, Write},
};

const SIZEX: isize = 101; // 11 101
const SIZEY: isize = 103; // 7 103

struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

// const EGG_X: f64 = 50.0;
// const EGG_Y: f64 = 51.0;
// const EGGNESS: f64 = 0.3;
// const EGG_WIDTH: f64 = EGG_X;
// const EGG_HEIGHT: f64 = EGG_Y;

impl Robot {
    fn step(&mut self) {
        let mut newpos = (
            self.position.0 as isize + self.velocity.0,
            self.position.1 as isize + self.velocity.1,
        );
        while newpos.0 < 0 {
            newpos.0 += SIZEX;
        }
        while newpos.0 >= SIZEX {
            newpos.0 -= SIZEX;
        }
        while newpos.1 < 0 {
            newpos.1 += SIZEY;
        }
        while newpos.1 >= SIZEY {
            newpos.1 -= SIZEY;
        }
        self.position = (newpos.0 as usize, newpos.1 as usize);
    }

    // fn in_easter_egg(&self) -> bool {
    //     let distance = (self.position.0 as f64 - EGG_X).powi(2)
    //         * (1.0 - EGGNESS * ((self.position.1 as f64 - EGG_HEIGHT) / EGG_HEIGHT))
    //         / EGG_WIDTH.powi(2)
    //         + (self.position.1 as f64 - EGG_Y).powi(2) / EGG_HEIGHT.powi(2);
    //     distance < 1.1
    // }

    fn in_tree(&self) -> bool {
        let x_diff = self.position.0.abs_diff(50);
        (x_diff * SIZEY as usize) < (self.position.1 * (SIZEX / 2) as usize + 40)
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    // let mut count: [usize; 4] = [0, 0, 0, 0];
    // const TIME: isize = 100;
    // const SIZEX_HALF: usize = SIZEX as usize / 2;
    // const SIZEY_HALF: usize = SIZEY as usize / 2;

    let mut robots: Vec<Robot> = Vec::new();

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
            robots.push(Robot {
                position: (px, py),
                velocity: (vx, vy),
            });
        }
    }
    for i in 0..100_000 {
        robots.iter_mut().for_each(|r| r.step());
        let outside = robots.iter().filter(|r| !r.in_tree()).count();
        // println!("{} {outside}", i+1);
        if outside < 100 {
            println!("{} {}", i + 1, outside);

            let mut board: [[u8; SIZEX as usize]; SIZEY as usize] =
                [array::from_fn(|_i| 0); SIZEY as usize];
            robots.iter().for_each(|r| {
                board[r.position.1][r.position.0] = if board[r.position.1][r.position.0] == 0 {
                    255
                } else {
                    board[r.position.1][r.position.0] / 3 * 2
                }
            });

            // for l in board.iter() {
            //     println!("{}", str::from_utf8(l).unwrap());
            // }

            use std::fs::File;
            use std::io::BufWriter;
            use std::path::Path;

            let os = OsString::from(&format!("P{}.png", i + 1));
            let path = Path::new(&os);
            let file = File::create(path).unwrap();
            let ref mut w = BufWriter::new(file);

            let mut encoder = png::Encoder::new(w, SIZEX as u32, SIZEY as u32);
            encoder.set_color(png::ColorType::Grayscale);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();

            let mut stream = writer.stream_writer().unwrap();
            for l in board.iter() {
                stream.write_all(l).unwrap();
            }
            stream.finish().unwrap();
        }
    }
    Ok(())
}
