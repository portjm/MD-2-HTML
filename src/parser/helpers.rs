#[derive(PartialEq)]
enum Elements {
    Heading(u8),
    Bold,
    Italics,
    Text,
    Empty
}



// pub fn parse_text(contents:&String) -> Option<String> {
//     let mut current_line = String::from("");
//     let mut result: Vec<String> = Vec::new();
//     let mut symbols = contents.chars();

//     let mut curr_element = Elements::Empty;
    
//     while let Some(symbol) = symbols.next() {
//         print!("{}", symbol);
//         match symbol {
//             '#' => {
//                 let element = heading(symbol, &curr_element);
                
//                     if let Elements::Heading(h) = curr_element{
//                         let html_elem = format!("<h{}>",h); 
//                         current_line.push_str(&html_elem);
//                     }
             
//             },
//             _ => {
//                 let next_element = heading(symbol, &curr_element);
//                 // if next_element != curr_element {

//                 // }
            
//             }
            
//         }
//     }


//     Some(current_line)
// }

#[derive(Debug,Copy, Clone)]
pub enum Tokens {
    HEAD1 = 0,
    HEAD2,
    HEAD3,
    HEAD4,
    HEAD5,
    HEAD6,
    EMPTY,
    ASTERISK,
    BLOCK,
    NUMBER,
    CHAR,
    PERIOD,
    DASH,
    BACKSLASH,
    NEWLINE,
    SPACE,
    CODE
    
}

#[derive(Debug)]
pub struct Token(Tokens, char);

fn heading(state: &Tokens) -> Option<Tokens>{
    match *state as isize {
        6 => return Some(Tokens::HEAD1),
        0  => return Some(Tokens::HEAD2),
        1  => return Some(Tokens::HEAD3),
        2  => return Some(Tokens::HEAD4),
        3  => return Some(Tokens::HEAD5),
        4  => return Some(Tokens::HEAD6),
        _ => None
    }
    
}

// Tokenizes MD document passed in as string
pub fn tokenize_md(contents: &String) -> Vec<Token> {
    let symbols:Vec<char> = contents.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    // Used to track state when token symbol consists of multiple chars (> 2)
    let mut current_token = Tokens::EMPTY;
    let mut idx:usize = 0;

    let mut current_symbol: char;

    while idx < symbols.len() {
        current_symbol = symbols[idx];
        // print!("{}", current_symbol);
        match current_symbol {
                    '#' => {
                        if let Some(new_token) = heading(&current_token) {
                            current_token = new_token; 
                        } 
                        
                        if symbols[idx + 1] != '#' {
                            tokens.push(Token(current_token, '#'));
                            current_token = Tokens::EMPTY;
                        }
                    },
                    '\r' => {
                        current_token = Tokens::EMPTY;
                        idx += 1;
                        continue;
                    },
                    '\n' => {
                        tokens.push(Token(Tokens::NEWLINE, current_symbol));
                        current_token = Tokens::EMPTY;
                    },
                    ' ' => tokens.push(Token(Tokens::SPACE, current_symbol)),
                    '*' => tokens.push(Token(Tokens::ASTERISK, current_symbol)),
                    '>' => tokens.push(Token(Tokens::BLOCK, current_symbol)),
                    '`' => tokens.push(Token(Tokens::CODE, current_symbol)),
                    '.' => tokens.push(Token(Tokens::PERIOD, current_symbol)),
                    '-' => tokens.push(Token(Tokens::DASH, current_symbol)),
                    '\\' => tokens.push(Token(Tokens::BACKSLASH, current_symbol)),
                    _ => { 
                        if current_symbol.is_ascii_digit() {
                            tokens.push(Token(Tokens::NUMBER, current_symbol));
                        } else {
                            tokens.push(Token(Tokens::CHAR, current_symbol));
                        }
                        current_token = Tokens::EMPTY;
                    }
                }
            
        idx += 1;
    }

    tokens
}