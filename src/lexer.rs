#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(i64),
    String(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Equal,
    Arrow, // ->
    LessDash, // <-
    Dot,
    Slash,
    BacktickBlock(String, String), // (tag, content)
    Underscore,
    EOF,
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '.' || c == '/' || c == ':'
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ch if ch.is_whitespace() => { chars.next(); },
            '/' => {
                // could be comment or slash operator
                chars.next();
                if let Some(&'/') = chars.peek() {
                    // line comment
                    chars.next();
                    while let Some(&nc) = chars.peek() {
                        chars.next();
                        if nc == '\n' { break; }
                    }
                } else if let Some(&'*') = chars.peek() {
                    // block comment
                    chars.next();
                    loop {
                        if let Some(nc) = chars.next() {
                            if nc == '*' {
                                if let Some(&'/') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                        } else { break; }
                    }
                } else {
                    tokens.push(Token::Slash);
                }
            }
            '(' => { chars.next(); tokens.push(Token::LParen); }
            ')' => { chars.next(); tokens.push(Token::RParen); }
            '{' => { chars.next(); tokens.push(Token::LBrace); }
            '}' => { chars.next(); tokens.push(Token::RBrace); }
            ',' => { chars.next(); tokens.push(Token::Comma); }
            ';' => { chars.next(); tokens.push(Token::Semicolon); }
            '=' => { chars.next(); tokens.push(Token::Equal); }
            '-' => {
                chars.next();
                if let Some(&'>') = chars.peek() {
                    chars.next(); tokens.push(Token::Arrow);
                } else {
                    // '-' alone treated as part of number or ignored
                }
            }
            '<' => {
                chars.next();
                if let Some(&'-') = chars.peek() {
                    chars.next(); tokens.push(Token::LessDash);
                }
            }
            '.' => { chars.next(); tokens.push(Token::Dot); }
            '`' => {
                // triple-backtick block: ```tag\ncontent```
                chars.next();
                if chars.peek() == Some(&'`') {
                    chars.next();
                    if chars.peek() == Some(&'`') {
                        chars.next();
                        // read optional tag until newline
                        let mut tag = String::new();
                        while let Some(&nc) = chars.peek() {
                            if nc == '\n' { chars.next(); break; }
                            if nc.is_whitespace() { chars.next(); break; }
                            tag.push(nc);
                            chars.next();
                        }
                        // read content until ```
                        let mut content = String::new();
                        loop {
                            if let Some(&nc) = chars.peek() {
                                if nc == '`' {
                                    // check for three
                                    let mut clone = chars.clone();
                                    clone.next();
                                    if clone.peek() == Some(&'`') {
                                        clone.next();
                                        if clone.peek() == Some(&'`') {
                                            // consume three
                                            chars.next(); chars.next(); chars.next();
                                            break;
                                        } else {
                                            content.push(chars.next().unwrap());
                                        }
                                    } else {
                                        content.push(chars.next().unwrap());
                                    }
                                } else {
                                    content.push(chars.next().unwrap());
                                }
                            } else { break; }
                        }
                        tokens.push(Token::BacktickBlock(tag, content));
                    }
                }
            }
            '_' => { chars.next(); tokens.push(Token::Underscore); }
            '"' => {
                chars.next();
                let mut s = String::new();
                while let Some(nc) = chars.next() {
                    if nc == '"' { break; }
                    s.push(nc);
                }
                tokens.push(Token::String(s));
            }
            ch if ch.is_digit(10) => {
                let mut num = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_digit(10) { num.push(nc); chars.next(); } else { break; }
                }
                if let Ok(n) = num.parse() { tokens.push(Token::Number(n)); }
            }
            ch if is_ident_start(ch) => {
                let mut id = String::new();
                id.push(ch);
                chars.next();
                while let Some(&nc) = chars.peek() {
                    if is_ident_char(nc) { id.push(nc); chars.next(); } else { break; }
                }
                tokens.push(Token::Ident(id));
            }
            _ => { chars.next(); }
        }
    }

    tokens.push(Token::EOF);
    tokens
}
