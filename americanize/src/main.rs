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
        let mut end: bool = false;
        if c.1 == ' ' || c.1.is_ascii_punctuation() || end {
            if c.0 == original.len()-1 && !c.1.is_ascii_punctuation() {end = true}
            if end { buffer.push(c.1);}
            if buffer.contains("tea") && buffer.len() == 3 {
                buffer = "coffee".to_string();
            } else if buffer.contains("Tea") && buffer.len() == 3 {
                buffer = "Coffee".to_string();
            }
            edited += &buffer;
            if !end {edited.push(c.1);}
            buffer.clear();
        } else {
            buffer.push(c.1);
        }
    }
    println!("{}", edited);
}