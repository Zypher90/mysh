use std::iter::Peekable;
use std::str::Chars;

pub enum Token {
    Word(String),
    Pipe,
    RedirectIn,
    RedirectOut,
    Append,
    Background
}

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&char) = chars.peek() {
        if char==' ' || char=='\t' {
            chars.next();
        }else{
            break;
        }
    }
}

fn read_word(chars: &mut Peekable<Chars>) -> String {
    let mut word = String::new();
    while let Some(&char) = chars.peek() {
        if char.is_whitespace() || "|<>&".contains(char) {
            break;
        }else if char == '"' {
            chars.next();
        }else{
            chars.next();
            word.push(char);
        }
    }

    word
}

fn read_single_quoted(chars: &mut Peekable<Chars>) -> String {
    let mut word = String::new();

    while let Some(&char) = chars.peek() {
        if char == '\'' {
            chars.next();
            break;
        }
        chars.next();
        word.push(char);
    }

    word
}

fn read_double_quoted(chars: &mut Peekable<Chars>) -> String {
    let mut word = String::new();

    while let Some(&char) = chars.peek() {
        if char == '\\' {
            chars.next();
            match chars.peek() {
                Some(&'\\') => {
                    chars.next();
                    word.push('\\');
                },
                Some(&'"') => {
                    chars.next();
                    word.push('"');
                },
                _ => {}
            }
        }else{
            chars.next();
            word.push(char);
        }
    }

    word
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    loop {
        skip_whitespace(&mut chars);

        match chars.peek() {
            None => break,
            Some(&'|') => {
                chars.next();
                tokens.push(Token::Pipe);
            },
            Some(&'&') => {
                chars.next();
                tokens.push(Token::Background);
            },
            Some(&'<') => {
                chars.next();
                tokens.push(Token::RedirectIn);
            },
            Some(&'>') => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Append);
                }else{
                    tokens.push(Token::RedirectIn);
                }
            },
            Some(&'\'') => {
                chars.next();
                tokens.push(Token::Word(read_single_quoted(&mut chars)));
            },
            Some(&'"') => {
                chars.next();
                tokens.push(Token::Word(read_double_quoted(&mut chars)));
            },
            Some(&'\\') => {
                chars.next();
            }
            _ => {
                tokens.push(Token::Word(read_word(&mut chars)));
            }
        }

    }

    tokens
}