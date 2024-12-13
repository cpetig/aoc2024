use std::io::{self, stdin, BufRead, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut count: usize = 0;
    let mut count2: usize = 0;
    let mut button_a: Option<(usize, usize)> = None;
    let mut button_b: Option<(usize, usize)> = None;

    for input in reader.lines().map_while(Result::ok) {
        if input.len() > 1 {
            let mut elem = input.split_whitespace();
            let typ = elem.next().unwrap();
            if typ == "Button" {
                let which = elem.next().unwrap();
                let x = elem.next().unwrap();
                let y = elem.next().unwrap();
                assert!(x.chars().next() == Some('X'));
                assert!(y.chars().next() == Some('Y'));
                assert!(x.chars().nth(1) == Some('+'));
                assert!(y.chars().nth(1) == Some('+'));
                assert!(x.chars().last() == Some(','));
                let x: usize = (x[2..x.len() - 1]).parse().unwrap();
                let y: usize = (y[2..]).parse().unwrap();
                if which == "A:" {
                    button_a = Some((x, y));
                } else if which == "B:" {
                    button_b = Some((x, y));
                } else {
                    unreachable!();
                }
            } else if typ == "Prize:" {
                let x = elem.next().unwrap();
                let y = elem.next().unwrap();
                // dbg!(&x[2..], &y[2..]);
                assert!(x.chars().next() == Some('X'));
                assert!(y.chars().next() == Some('Y'));
                assert!(x.chars().nth(1) == Some('='));
                assert!(y.chars().nth(1) == Some('='));
                assert!(x.chars().last() == Some(','));
                let x: usize = x[2..x.len() - 1].parse().unwrap();
                let y: usize = y[2..].parse().unwrap();
                let (Some(a), Some(b)) = (button_a, button_b) else {
                    unreachable!()
                };
                let (adx, ady) = (a.0 as f64, a.1 as f64);
                let (bdx, bdy) = (b.0 as f64, b.1 as f64);
                let (xd, yd) = (x as f64, y as f64);
                // dbg!((yd, adx, ady, xd, bdy, adx, ady, bdx));
                // gaussian elimination
                // (x,y) = (ax bx ; ay by)(a,b)
                let numb = (yd * adx - xd * ady) / (bdy * adx - bdx * ady);
                // x = ax a + bx * b
                // a = (x - bx * b)/ax
                let numa = (xd - bdx * numb) / adx;
                // dbg!(numa, numb);
                if (0.0..100.0).contains(&numa) && (0.0..100.0).contains(&numb) {
                    let numa = numa as usize;
                    let numb = numb as usize;
                    if x == numa * a.0 + numb * b.0 && y == numa * a.1 + numb * b.1 {
                        count += numa * 3 + numb;
                    }
                }

                let (xd2, yd2) = (x as f64 + 10000000000000.0, y as f64  + 10000000000000.0);
                // dbg!((yd, adx, ady, xd, bdy, adx, ady, bdx));
                // gaussian elimination
                // (x,y) = (ax bx ; ay by)(a,b)
                let numb2 = (yd2 * adx - xd2 * ady) / (bdy * adx - bdx * ady);
                // x = ax a + bx * b
                // a = (x - bx * b)/ax
                let numa2 = (xd2 - bdx * numb2) / adx;
                // dbg!(numa, numb);
                if numa2 > 0.0 && numb2>0.0 {
                    let numa = numa2.round() as usize;
                    let numb = numb2.round() as usize;
                    if x+10000000000000 == numa * a.0 + numb * b.0 && y+10000000000000 == numa * a.1 + numb * b.1 {
                        count2 += numa * 3 + numb;
                    }
                }
            } else {
                unreachable!();
            }
        }
    }
    println!("{count} {count2}");
    Ok(())
}
