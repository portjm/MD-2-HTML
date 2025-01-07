#![allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq)]
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
    CODE,
}

#[derive(Debug, Clone, Copy)]
pub struct Token(Tokens, char);

fn heading(state: &Tokens) -> Option<Tokens> {
    match *state as isize {
        6 => return Some(Tokens::HEAD1),
        0 => return Some(Tokens::HEAD2),
        1 => return Some(Tokens::HEAD3),
        2 => return Some(Tokens::HEAD4),
        3 => return Some(Tokens::HEAD5),
        4 => return Some(Tokens::HEAD6),
        _ => None,
    }
}

// Tokenizes MD document passed in as string
pub fn tokenize(contents: &String) -> Vec<Token> {
    // File as char vec
    let symbols: Vec<char> = contents.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    // Used to track state when token symbol consists of multiple chars (> 2)
    let mut current_token = Tokens::EMPTY;
    let mut idx: usize = 0;

    let mut current_symbol: char;

    while idx < symbols.len() {
        current_symbol = symbols[idx];

        match current_symbol {
            // TODO: NEEDS TO ACCOUNT FOR SPACE AFTER '#' ELSE INVALID HEADER
            '#' => {
                if let Some(new_token) = heading(&current_token) {
                    current_token = new_token;
                }

                if symbols[idx + 1] != '#' {
                    tokens.push(Token(current_token, '#'));
                    current_token = Tokens::EMPTY;
                }
            }
            '\r' => {
                current_token = Tokens::EMPTY;
                idx += 1;
                continue;
            }
            '\n' => {
                tokens.push(Token(Tokens::NEWLINE, current_symbol));
                current_token = Tokens::EMPTY;
            }
            ' ' => tokens.push(Token(Tokens::SPACE, current_symbol)),
            '*' => {
                if symbols[idx + 1] != '*' {
                    tokens.push(Token(Tokens::ASTERISK, current_symbol));
                } else {
                    tokens.push(Token(Tokens::DBLASTERISK, current_symbol));
                }
            }
            '>' => tokens.push(Token(Tokens::BLOCK, current_symbol)),
            '`' => tokens.push(Token(Tokens::CODE, current_symbol)),
            //'.' => tokens.push(Token(Tokens::PERIOD, current_symbol)),
            '-' => tokens.push(Token(Tokens::DASH, current_symbol)),

            '\\' => match symbols[idx + 1] {
                '*' | '>' | '`' | '-' => {
                    tokens.push(Token(Tokens::CHAR, symbols[idx + 1]));
                    idx += 1;
                }
                _ => tokens.push(Token(Tokens::BACKSLASH, current_symbol)),
            },

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
    Empty,
}

impl Element {
    fn to_html(&self) -> String {
        match self {
            Element::Heading { level, content } => format!("<h{}>{}</h{}>", level, content, level),
            Element::Italics(text) => format!("<em>{}</em>", text),
            _ => String::from("placeholder"),
        }
    }
}

struct Parser {
    markdown: Vec<Token>,
    current_idx: usize,
    current_branch: Vec<Element>,
    document: Vec<String>,
}

impl Parser {
    // Initialize new Parser with Empty element (default state)
    fn new(md_tokens: Vec<Token>) -> Self {
        let mut np = Parser {
            markdown: md_tokens,
            current_idx: 0,
            current_branch: Vec::new(),
            document: Vec::new(),
        };

        np.current_branch.push(Element::Empty);

        return np;
    }

    fn transition(&mut self, input: Token) {
        if let Some(current_state) = self.current_branch.last_mut() {
            match current_state {
                Element::Empty => {
                    // Heading token
                    if (input.0 as u8) < 6 {
                        *current_state = Element::Heading {
                            level: (input.0 as u8) + 1,
                            content: "".to_string(),
                        }
                    }
                    // TODO: Implement Block token (text, list, block quote)
                }

                Element::Heading { level, content } => {
                    // Regular text
                    if input.0 == Tokens::CHAR
                        || input.0 == Tokens::NUMBER
                        || input.0 == Tokens::DASH
                        || input.0 == Tokens::SPACE
                    {
                        content.push(input.1);
                    // Inline elements
                    } else if input.0 == Tokens::ASTERISK {
                        let mut inline_elem = Element::Italics(String::new());
                        let new_idx = self.parse_inline(inline_elem);

                        // content.push_str(&inline_elem.to_html());
                    } else if input.0 == Tokens::NEWLINE {
                        self.document.push(current_state.to_html());
                        *current_state = Element::Empty;
                    }
                }
                _ => {}
            }
            self.current_idx += 1;
        }
    }

    fn parse_inline(&self, elem: Element) -> usize {
        // let mut text: String = String::new();
        let mut current_idx = self.current_idx;
        if let Element::Italics(mut text) = elem {
            loop {
                current_idx += 1;
                let current_token = &self.markdown[current_idx];

                match current_token.0 {
                    Tokens::CHAR | Tokens::NUMBER | Tokens::DASH | Tokens::SPACE => {
                        text.push(current_token.1);
                    }
                    Tokens::ASTERISK => {
                        break;
                    }
                    _ => {
                        println!("Unknown token.");
                    }
                }
            }
        }
        return current_idx;
    }

    fn to_html(&mut self) {
        let mut elements = self.document.iter();
        while let Some(elem) = elements.next() {
            println!("{}", elem);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_tokenization() {
        for level in 1..7 {
            // Generate MD element
            let hashes = "#".repeat(level);
            let header_text = format!("{} Title {}", hashes, level);

            let tokens = tokenize(&header_text);

            // Header enum variants start at 0, hence level - 1
            assert_eq!(tokens[0].0 as usize, level - 1);
        }
    }

    #[test]
    fn test_heading_parsing() {
        let heading = String::from("## title\n ### title3\n");
        let tokens = tokenize(&heading);
        let n = tokens.len();

        let mut prsr = Parser::new(tokens);
        while prsr.current_idx < n {
            let t = prsr.markdown[prsr.current_idx];
            println!("{:?}", t);
            prsr.transition(t);
        }

        prsr.to_html();
    }

    // INCOMPLETE
    // #[test]
    // fn test_italics() {
    //     let valid_case = String::from("*test text*");
    //     let invalid_case = String::from("* test text*");

    //     let valid_tokens = tokenize(&valid_case);
    //     let invalid_tokens = tokenize(&invalid_case);
    // }
}
