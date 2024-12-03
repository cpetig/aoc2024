use std::io::{self, stdin, BufReader, Read};

enum State {
    Initial,
    Mul {
        letters: u32,
    },
    FirstNumber {
        digits: u32,
        value: u32,
    },
    SecondNumber {
        digits: u32,
        value: u32,
        first_value: u32,
    },
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(stdin());

    let mut valid_part = State::Initial;
    let mut sum = 0;

    for input in reader.bytes() {
        let Ok(input) = input else {
            break;
        };
        match valid_part {
            State::Mul { letters } => {
                valid_part = if input == b"mul("[letters as usize] {
                    if letters == 3 {
                        State::FirstNumber {
                            digits: 0,
                            value: 0,
                        }
                    } else {
                        State::Mul {
                            letters: letters + 1,
                        }
                    }
                } else {
                    State::Initial
                }
            }
            State::FirstNumber { digits, value } => {
                valid_part = if input.is_ascii_digit() && digits < 3 {
                    State::FirstNumber {
                        digits: digits + 1,
                        value: value * 10 + ((input - b'0') as u32),
                    }
                } else if input == b',' && digits > 0 {
                    State::SecondNumber {
                        digits: 0,
                        value: 0,
                        first_value: value,
                    }
                } else {
                    State::Initial
                }
            }
            State::SecondNumber {
                digits,
                value,
                first_value,
            } => {
                valid_part = if input.is_ascii_digit() && digits < 3 {
                    State::SecondNumber {
                        digits: digits + 1,
                        value: value * 10 + ((input - b'0') as u32),
                        first_value,
                    }
                } else if input == b')' && digits > 0 {
                    sum += first_value * value;
                    State::Initial
                } else {
                    State::Initial
                }
            }
            State::Initial => { /* handle later */ }
        }
        // as this might result from a reset we try it again after the previous handling
        if matches!(valid_part, State::Initial) && input == b'm' {
            valid_part = State::Mul { letters: 1 };
        }
    }
    println!("{sum}");
    Ok(())
}
