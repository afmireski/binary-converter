use std::io;

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

fn get_bit_len(number: i32) -> u8 {
    let mut i: u8 = 0;

    while number >= 2_i32.pow(i.into()) {
        i += 1
    }

    return i;
}

fn convert_int_to_binary(number: i32) -> String {
    let bits = get_bit_len(number);

    let mut result = String::new();

    if number == 0 {
        return String::from("0");
    }

    let mut max_value = 2_i32.pow(bits.into());
    let mut value = number;

    while value > 0 {
        if value >= max_value {
            result.push('1');
            value -= max_value;
        } else {
            result.push('0');
        }
        max_value /= 2;
    }

    if &result[0..1] == "0" {
        result = (&result[1..]).to_string();
    }

    while result.len() < bits.into() {
        result.push('0');
    }

    result
}

fn convert_decimal_to_binary(number: f32) -> String {
    if number == 0.0 {
        return String::from("0");
    }

    let mut result = String::new();

    let mut value = number;
    while value != 0.0 {
        value *= 2.0;

        if value >= 1.0 {
            result.push('1');
            value -= 1.0;
        } else if value < 1.0 {
            result.push('0');
        }
    }

    result
}

fn convert_float_to_binary(mut number: f32) {
    let mut has_signal: u8 = 0;

    if number < 0.0 {
        has_signal = 1;
        number = -number;
    }

    let int_value = get_int_value(number);
    let float_value = number - (int_value as f32);

    let binary_int_value = convert_int_to_binary(int_value);
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

    convert_float_to_binary(input);
}
