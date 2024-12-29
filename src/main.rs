use std::{env, process, fs, io, path::Path};
pub mod parser;
// use parser::helpers::parse_text;
use parser::helpers::tokenize_md;
fn read_md_as_string(fp: &Path) -> Result<String, io::Error> {
    fs::read_to_string(fp)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: filepath not specified.\nUsage: {} <filepath>", args[0]);
        process::exit(1);
    } 
    let file_type = &args[1][args[1].len()-3..];
    if file_type != ".md" && file_type != ".MD" {
        eprintln!("Error: Invalid file type: {}", args[1]);
        process::exit(1);
    }

    let filepath = Path::new(&args[1]);

    match read_md_as_string(filepath) {
        Ok(contents) => {
            // parse_text(&contents);
            println!("File contents:\n{}", contents);
            let tokens = tokenize_md(&contents);
            for t in tokens{
                println!("{:?}", &t);
            }
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }

}
