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
        "1.1" => solutions::_01::part_1("src/solutions/_01/_01.txt").to_string(),
        "1.2" => solutions::_01::part_2("src/solutions/_01/_01.txt").to_string(),
        "2.1" => solutions::_02::part_1("src/solutions/_02/_02.txt").to_string(),
        "2.2" => solutions::_02::part_2("src/solutions/_02/_02.txt").to_string(),
        "3.1" => solutions::_03::part_1("src/solutions/_03/real.txt").to_string(),
        "3.2" => solutions::_03::part_2("src/solutions/_03/real.txt").to_string(),
        &_ => "Not valid".to_string(),
    };

    println!(
        "Solution => {}\nTook about => {:?}",
        solution,
        start.elapsed()
    );
}
