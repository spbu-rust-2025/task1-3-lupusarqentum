use std::io;
use std::fs;

fn main() {
    let mut input = String::new();
    let line_reading_result = io::stdin().read_line(&mut input);
    match line_reading_result {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read a line from stdin!");
            return;
        }
    }
    let filename = input.trim();
    match fs::read(filename) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure")
    }
}
