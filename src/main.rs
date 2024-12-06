use std::error;
use std::fmt::Error;
use std::io::{self, BufRead};
use std::fs;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let example_solution = 11;
    assert_eq!(findstar("src/example").unwrap(),example_solution);
    println!("Found the correct example solution {example_solution}.");

    let answer = findstar("src/input").unwrap();
    println!("Answer: {answer}");
}

fn findstar(file_path: &str) -> Result<isize,Box<dyn error::Error>> {
    //let contents = fs::read_to_string(file_path)
    //    .expect("Should have been able to read the file");

    let file = fs::File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();


    let solution = 11;
    Ok(solution)
}
