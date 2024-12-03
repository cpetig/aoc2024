use std::io::{self, stdin, BufReader, Read};

// #[derive(Debug)]
enum State {
    Initial,
    Mul {
        letters: u32,
    },
    Do {
        letters: u32,
    },
    Dont {
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
    let mut sum_enabled = 0;
    let mut enabled = true;

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
            State::Do { letters } => {
                valid_part = if input == b"do()"[letters as usize] {
                    if letters == 3 {
                        enabled = true;
                        State::Initial
                    } else {
                        State::Do {
                            letters: letters + 1,
                        }
                    }
                } else if letters == 2 && input == b'n' {
                    State::Dont {
                        letters: letters + 1,
                    }
                } else {
                    State::Initial
                };
                // dbg!((&valid_part, letters, input));
            }
            State::Dont { letters } => {
                valid_part = if input == b"don't()"[letters as usize] {
                    if letters == 6 {
                        enabled = false;
                        State::Initial
                    } else {
                        State::Dont {
                            letters: letters + 1,
                        }
                    }
                } else {
                    State::Initial
                };
                // dbg!((&valid_part, letters, input));
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
                    if enabled {
                        sum_enabled += first_value * value;
                    }
                    State::Initial
                } else {
                    State::Initial
                }
            }
            State::Initial => { /* handle later */ }
        }
        // as this might result from a reset we try it again after the previous handling
        match (&valid_part, input) {
            (State::Initial, b'm') => valid_part = State::Mul { letters: 1 },
            (State::Initial, b'd') => valid_part = State::Do { letters: 1 },
            (_, _) => (),
        }
    }
    println!("{sum} {sum_enabled}");
    Ok(())
}
