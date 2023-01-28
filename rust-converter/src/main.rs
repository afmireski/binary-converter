use std::io;

fn main() {
    println!("###############################################");
    println!("# Binary Converter - RUST\n\n");

    let mut input = String::new();

    println!("Type any float number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f32 = input.trim().parse().expect("Please try again and type a float number");
}
