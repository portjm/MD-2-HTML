#[derive(Debug,Copy, Clone, PartialEq)]
pub enum Tokens {
    HEAD1 = 0,
    HEAD2,
    HEAD3,
    HEAD4,
    HEAD5,
    HEAD6,
    EMPTY,
    ASTERISK,
    DBLASTERISK,
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
pub fn tokenize(contents: &String) -> Vec<Token> {
    // File as char vec
    let symbols:Vec<char> = contents.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    // Used to track state when token symbol consists of multiple chars (> 2)
    let mut current_token = Tokens::EMPTY;
    let mut idx:usize = 0;

    let mut current_symbol: char;

    while idx < symbols.len() {
        current_symbol = symbols[idx];

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
                    '*' => {
                        if symbols[idx+1] != '*' {
                            tokens.push(Token(Tokens::ASTERISK, current_symbol));
                        } else {
        
                            tokens.push(Token(Tokens::DBLASTERISK, current_symbol));
                        }
                        
                    },
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

#[derive(PartialEq)]
enum Element {
    Heading { level: u8, content: String },
    Paragraph(Vec<Element>), // Inline elements (e.g., text, bold) as children
    Bold(String),
    PartialBold(String),
    Italics(String),
    PartialItalics(String),
    Code(String),
    Text(String),
    Empty
}

impl Element {
    fn to_html(&self) -> String {
        match self {
            Element::Italics(text) => format!("<em>{}</em>", text),
            _ => String::from("placeholder")
        }
    }
}

struct Parser {
    current_state: Element,
}

impl Parser {
    fn new() -> Self {
        Parser {
            current_state: Element::Empty
        }
    }

    fn transition(&mut self, input:Token) {
        match &mut self.current_state {
            // Base state
            Element::Empty => {
                if input.0 == Tokens::ASTERISK {
                    self.current_state = Element::PartialItalics("".to_string());
                } else if input.0 == Tokens::DBLASTERISK {
                    self.current_state = Element::Bold("".to_string());
                }
            },

            Element::PartialItalics(text) => {
                if input.0 == Tokens::ASTERISK {
                    self.current_state = Element::Bold(text.to_owned());
                } else {
                    self.current_state = Element::PartialItalics(text.to_owned());
                }
            },
            Element::Italics(text) => {
                if input.0 != Tokens::ASTERISK || input.0 != Tokens::NEWLINE  {
                    text.push(input.1);
                } else if input.0 == Tokens::ASTERISK { // Complete Italics element
                    // add to parent element
                }
            }

            Element::Bold(text) => {

            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_headers() {
        for level in 1..7 {
            // Generate MD element
            let hashes = "#".repeat(level);
            let header_text = format!("{} Title {}", hashes, level);
            
            let tokens = tokenize(&header_text);
            
            // Header enum variants start at 0, hence level - 1
            assert_eq!(tokens[0].0 as usize, level - 1);
        }
    }
}