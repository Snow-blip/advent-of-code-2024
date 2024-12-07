use std::path::absolute;
use std::{error, isize};
use std::fmt::Error;
use std::io::{self, BufRead};
use std::fs;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let example_solution = 31;
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

    let hm0 = count_element_function(v0);
    let hm1 = count_element_function(v1);

    let mut similarity_score = 0;
    for (number, frequency0) in hm0.iter() {
        let frequency1 = hm1.get(number).unwrap_or(&0);
        similarity_score += *number * *frequency0 as isize * *frequency1 as isize;
    };

    let solution = similarity_score;
    Ok(solution)
}

use std::collections::HashMap;
fn count_element_function<I>(it: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut result = HashMap::new();

    for item in it {
        *result.entry(item).or_insert(0) += 1;
    }

    result
}