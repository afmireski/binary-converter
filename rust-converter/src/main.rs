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

fn convert_int_to_binary_excess_127(expoent: i16) -> String {
    if -127 <= expoent && expoent <= 128 {
        let mut value: i16 = 255;

        let mut max_value: i16 = 128;

        let mut result = String::new();

        while value > 0 {
            if value >= max_value {
                result.push('0');
                value -= max_value;
            } else {
                result.push('0');
            }
            max_value /= 2;
        }

        while result.len() < 8 {
            result.push('0');
        }

        return result;
    } else {
        return String::from("00000000");
    }
}

fn calculate_exponents(int_binary: &str, decimal_binary: &str) -> (i16, String) {
    let mut int_exponent: i16 = -127;

    if int_binary.contains('1') {
        let len = int_binary.len();

        let mut last_index: usize = len;
        for (i, c) in int_binary.chars().enumerate() {
            if c == '1' {
                last_index = len - (i + 1);
                break;
            }
        }

        int_exponent = last_index as i16;
    } else if decimal_binary.contains('1') {
        let len = decimal_binary.len();

        let mut last_index: usize = len-1;
        for (i, c) in int_binary.chars().enumerate() {
            if c == '1' {
                last_index = i + 1;
                break;
            }
        }

        int_exponent = -(last_index as i16);
    }

    let binary_exponent = convert_int_to_binary_excess_127(int_exponent);

    return (int_exponent, binary_exponent);
}

fn calculate_mantissa(int_binary: &str, decimal_binary: &str, exponent: i16) -> String {
    let mut mantissa = String::new();

    if exponent > 0 {
        let length = int_binary.len();

        let start = length - (exponent as usize);

        let mut merged_binary = String::new();
        merged_binary.push_str(int_binary);
        merged_binary.push_str(decimal_binary);

        let mut i = start;
        while mantissa.len() < 23 {
            if i < merged_binary.len() {
                mantissa.push_str(&merged_binary[i..i + 1]);

                i += 1;
            } else {
                if decimal_binary.len() == 0 || decimal_binary == "0" {
                    mantissa.push('0');
                } else {
                    i = start;
                }
            }
        }
    } else {
        let start = exponent.abs() as usize;

        let mut i = start;
        while mantissa.len() < 23 {
            if decimal_binary.len() == 0 || decimal_binary == "0" {
                mantissa.push('0');
            } else {
                mantissa.push_str(&decimal_binary[i..i + 1]);

                i += 1;

                if i == decimal_binary.len() {
                    i = start;
                }
            }
        }
    }

    mantissa
}

fn convert_float_to_binary(mut number: f32) -> String {
    let mut has_signal: char = '0';

    if number < 0.0 {
        has_signal = '1';
        number = -number;
    }

    let int_value = get_int_value(number);
    let float_value = number - (int_value as f32);

    let binary_int_value = convert_int_to_binary(int_value);
    let binary_decimal_value = convert_decimal_to_binary(float_value);

    let (int_exponent, bin_exponent) =
        calculate_exponents(&binary_int_value, &binary_decimal_value);

    let mantissa = calculate_mantissa(&binary_int_value, &binary_decimal_value, int_exponent);

    let mut response = String::new();
    response.push(has_signal);
    response.push_str(&bin_exponent);
    response.push_str(&mantissa);

    response
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

    let response = convert_float_to_binary(input);
    
    println!("\n\n\tStored {}: \n\t[{} - {}]", input, response, response.len());
    println!("###############################################");
}
