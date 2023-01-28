use std::io;
use std::str;

fn get_int_value(n: f32) -> i32 {
    let s = n.to_string();

    let mut slice = &s[..];
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b'.' {
            slice = &s[0..i];
            break;
        }
    }

    slice.parse::<i32>().expect("Fail on get int value")
}

fn convertFloatToBinary(mut number: f32) {
    let mut has_signal: u8 = 0;

    if number < 0.0 {
        has_signal = 1;
        number = -number;
    }

    let int_value = get_int_value(number);
    let float_value = number - (int_value as f32);
}

fn main() {
    println!("###############################################");
    println!("# Binary Converter - RUST\n\n");

    let mut input = String::new();

    println!("Type any float number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f32 = input
        .trim()
        .parse()
        .expect("Please try again and type a float number");
}
