use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if env::args().len() < 2 {
        println!("Not enough command line arguments");
        exit(0);
    }
    let original: String = args[1].clone();
    let mut edited: String = "".to_string();
    let mut buffer: String = "".to_string();
    for c in original.char_indices() { 
        if c.1 == ' ' || c.1.is_ascii_punctuation(){
            if buffer.contains("tea") && buffer.len() == 3 {
                buffer = "coffee".to_string();
            } else if buffer.contains("Tea") && buffer.len() == 3 {
                buffer = "Coffee".to_string();
            }
            buffer.push(c.1);
            edited += &buffer;
            buffer.clear();
        } else {
            buffer.push(c.1);
        }
        if c.0 == original.len() -1 {
            if buffer.contains("tea") && buffer.len() == 3 {
                buffer = "coffee".to_string();
            } else if buffer.contains("Tea") && buffer.len() == 3 {
                buffer = "Coffee".to_string();
            }
            edited += &buffer;
        }
    }
    println!("{}!", edited);
}