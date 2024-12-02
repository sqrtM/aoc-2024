use std::io;
use std::time::Instant;

mod solutions;

fn main() {
    let mut input = String::new();

    println!("Select question to solve:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let start = Instant::now();

    let solution = match input.trim() {
        "01" => solutions::_01::solve().to_string(),
        &_ => "Not valid".to_string(),
    };

    println!(
        "Solution => {}\nTook about => {:?}",
        solution,
        start.elapsed()
    );
}
