use std::path::absolute;
use std::{error, isize};
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

    let mut v0: Vec<isize> = Vec::new();
    let mut v1: Vec<isize> = Vec::new();
    for line in lines.flatten() {
        let mut numbers = line.split(' ').map(|s| s.parse::<isize>()).flatten();
        v0.push(numbers.next().unwrap());
        v1.push(numbers.next().unwrap());
    };

    v0.sort();
    v1.sort();

    let mut sum = 0;
    for (&mut ai, &mut bi) in v0.iter_mut().zip(v1.iter_mut()) {
        sum += (ai-bi).abs();
    }
    
    let solution = sum;
    Ok(solution)
}
