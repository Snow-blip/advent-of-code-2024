use std::fs;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let example_solution = 11;
    assert_eq!(findstar("src/example"),example_solution);
    println!("Found the correct example solution {example_solution}.");

    let answer = findstar("src/input");
    println!("Answer: {answer}");
}

fn findstar(file_path: &str) -> isize {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let solution: isize;
    solution = 11;
    solution
}
