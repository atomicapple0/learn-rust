use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args[1].clone();
    let filename = args[2].clone();

    let contents = fs::read_to_string(filename).expect("wtf happened to the file");

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    for line in results {
        println!("{}", line);
    }
}
