use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./read_file <filename>");
        return;
    }
    let filename = &args[1];
    let text = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("{}", text);
}
