use super::tokens::Token;
#[derive(Debug)]
pub struct Command {
    argv: Vec<String>,
    file_stdin: Option<String>,
    file_stdout: Option<String>,
    append: bool
}

impl Command {
    fn default() -> Self {
        Command {
            argv: Vec::new(),
            file_stdin: None,
            file_stdout: None,
            append: false
        }
    }
}

#[derive(Debug)]
pub struct Pipeline {
    commands: Vec<Command>,
    background_status: bool
}

impl Pipeline {
    fn new() -> Self {
        Pipeline {
            commands: Vec::new(),
            background_status: false
        }
    }
}

#[derive(Debug)]
pub enum ParseError{
    FileNotProvided,
    EmptyCommand
}

pub fn parse_tokens(tokens: &[Token]) -> Result<Pipeline, ParseError> {
    let mut pipeline = Pipeline::new();
    let mut current = Command::default();
    let mut i=0;

    while(i < tokens.len()) {
        match &tokens[i] {
            Token::RedirectOut => {
                if i+1 >= tokens.len() {
                    eprintln!("No filename provided");
                    current.file_stdout = None;
                    return Err(ParseError::FileNotProvided);
                }
                if let (Token::Word(filename)) = &tokens[i+1] {
                    current.file_stdout = Some(filename.clone());
                    current.append = false;
                }
                i+=1;
            },
            Token::RedirectIn => {
                if i+1 >= tokens.len() {
                    eprintln!("No filename provided");
                    current.file_stdin = None;
                    return Err(ParseError::FileNotProvided);
                }
                if let (Token::Word(filename)) = &tokens[i+1] {
                    current.file_stdin = Some(filename.clone());
                    current.append = false;
                }
                i+=1;
            },
            Token::Append => {
                if i+1 >= tokens.len() {
                    eprintln!("No filename provided");
                    current.file_stdout = None;
                    return Err(ParseError::FileNotProvided);
                }
                if let (Token::Word(filename)) = &tokens[i+1] {
                    current.file_stdout = Some(filename.clone());
                    current.append = true;
                }
                i+=1;
            },
            Token::Pipe => {
                if !current.argv.is_empty() {
                    pipeline.commands.push(current);
                    current = Command::default();
                }
            },
            Token::Background => {
                pipeline.background_status = true;
            },
            Token::Word(s) => {
                current.argv.push(s.clone());
            }
        }

        i += 1;
    }

    if !current.argv.is_empty() {
        pipeline.commands.push(current);
    }

    Ok(pipeline)
}