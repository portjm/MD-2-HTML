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
    ITAL,
    BOLD,
    BLOCK,
    NUMBER,
    CHAR,
    NEWLINE,
    WHTSPACE,
    EMPTY
}

#[derive(Debug)]
pub struct Token(Tokens, char);

fn heading(state: &Tokens) -> Option<Tokens>{
    match *state as isize {
        0  => return Some(Tokens::HEAD2),
        1  => return Some(Tokens::HEAD3),
        2  => return Some(Tokens::HEAD4),
        3  => return Some(Tokens::HEAD5),
        4  => return Some(Tokens::HEAD6),
        _ => None
    }
    
}

pub fn tokenize_md(contents: &String) -> Vec<Token> {
    let mut symbols = contents.chars();
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_token = Tokens::EMPTY;

    while let Some(symbol) = symbols.next() {
        match symbol {
            '#' => {
                if current_token as isize > 5{
                    current_token = Tokens::HEAD1;
                    tokens.push(Token(current_token, symbol));
                } else if let Some(new_token) = heading(&current_token) {
                    tokens.pop();
                    current_token = new_token;   
                    tokens.push(Token(current_token, symbol));
                } else {
                    tokens.pop();
                    tokens.push(Token(Tokens::CHAR, symbol));
                    current_token = Tokens::EMPTY;
                }
            },
            _ => { tokens.push(Token(Tokens::CHAR, symbol))}
        }
    }

    tokens
}