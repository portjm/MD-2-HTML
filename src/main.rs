use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: filepath not specified.\nUsage: {} <filepath>", args[0]);
        process::exit(1);
    } 
    let file_type = &args[1][args[1].len()-3..];
    if file_type != ".md"{
        eprintln!("Error: Invalid file type: {}", args[1]);
        process::exit(1);
    }
    println!("Num args: {}", args.len());
    println!("arg 1: {}", file_type);
}
