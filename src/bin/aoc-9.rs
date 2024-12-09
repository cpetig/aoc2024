use std::{
    io::{self, stdin, BufReader, Read},
    iter,
};

fn sum_over(mut low: usize, high: usize) -> usize {
    if low == 0 {
        low = 1
    };
    (high * (high + 1) - (low - 1) * (low)) / 2
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(stdin());

    let mut disk = String::new();
    reader.read_to_string(&mut disk)?;
    let disk = disk.as_bytes();
    let mut sum: usize = 0;
    let mut fillindex = (disk.len() - 1) / 2;
    let mut fillcount = (disk[fillindex * 2] - b'0') as usize;
    let mut countindex = 0;
    let mut outposition = 0;

    // dbg!(sum_over(3,5));
    // dbg!(sum_over(0,5));

    loop {
        let count = if countindex == fillindex {
            fillcount
        } else {
            (disk[countindex * 2] - b'0') as usize
        };
        sum += sum_over(outposition, outposition + count - 1) * countindex;
        // println!("A {outposition} {countindex} {count} {sum}");
        outposition += count;

        if countindex < fillindex {
            let mut filler = (disk[countindex * 2 + 1] - b'0') as usize;
            while filler > 0 {
                if fillcount > filler {
                    let used = filler;
                    sum += sum_over(outposition, outposition + used - 1) * fillindex;
                    // println!("B {outposition} {fillindex} {used} {sum}");
                    outposition += used;
                    fillcount -= used;
                    filler -= used;
                } else {
                    sum += sum_over(outposition, outposition + fillcount - 1) * fillindex;
                    // println!("C {outposition} {fillindex} {fillcount} {sum}");
                    outposition += fillcount;
                    filler -= fillcount;
                    // retrieve the next filling info
                    fillindex -= 1;
                    fillcount = (disk[fillindex * 2] - b'0') as usize;
                }
            }
            countindex += 1;
        } else {
            break;
        }
    }
    println!("{sum}");

    // (position, size)
    let mut files: Vec<(usize, usize)> = Vec::new();
    let mut gaps: Vec<(usize, usize)> = Vec::new();
    let mut position = 0;
    let mut iterator = disk.iter();
    loop {
        let Some(&size) = iterator.next() else {
            break;
        };
        if size < b'0' || size > b'9' {
            break;
        }
        let size = (size - b'0') as usize;
        files.push((position, size));
        position += size;

        let Some(&size) = iterator.next() else {
            break;
        };
        if size < b'0' || size > b'9' {
            break;
        }
        let size = (size - b'0') as usize;
        gaps.push((position, size));
        position += size;
    }
    // try to move each file once left
    for file in files.iter_mut().rev() {
        for gap in gaps.iter_mut() {
            if gap.0 >= file.0 {
                break;
            }
            if gap.1 >= file.1 {
                file.0 = gap.0;
                gap.0 += file.1;
                gap.1 -= file.1;
                break;
            }
        }
    }
    // dbg!(&files);
    let sum2 = files
        .iter()
        .enumerate()
        .fold(0_usize, |acc, (n, &(pos, size))| {
            acc + n * sum_over(pos, pos + size - 1)
        });
    println!("{sum2}");
    Ok(())
}
