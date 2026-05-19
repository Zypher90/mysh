use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::decl::parser::{Command, Pipeline};
use super::resolution::{resolve_path, list_dir};

fn change_dir(commands: &Vec<String>) {
    let path = Path::new(commands[1].as_str());

    if !env::set_current_dir(path).is_ok() {
        eprintln!("Invalid path provided");
    }
}

fn echo_builtin(command : &Command) {
    if let Some(file_path) = &command.file_stdin {
        let output = command.argv.
            get(1);
        let file = File::open(file_path);
        let Ok(mut file) = file else{
            eprintln!("Invalid file path");
            return;
        };
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("");
        eprintln!("{}{}", output.unwrap_or(&String::new()), buf);
        return;
    }

    if let Some(file_path) = &command.file_stdout {
        let output = command.argv.
            get(1);
        let mut file: std::io::Result<File>;
        if(command.append) {
            file = OpenOptions::new().append(true).open(file_path);
        }else{
            file = File::create(file_path);
        }
        let Ok(mut file) = file else{
            eprintln!("Invalid file path");
            return;
        };
        eprintln!("{}", output.unwrap_or(&"".to_string()));
        file.write_all(output.unwrap_or(&"".to_string()).as_bytes()).expect("Unsuccessful file write operation");
        return;
    }

    let output = command.argv.get(1);
    eprintln!("{}", output.unwrap_or(&"".to_string()));
}

fn execute_external(command: &mut Command) {
    let program = resolve_path(&command.argv[0]);
    let mut child = match program {
        Some(t) => {
            std::process::Command::new(&t)
                .args(&command.argv[1..])
                .spawn()
                .expect("Failed to spawn command")
        }
        None => {
            command.argv.insert(0, "/C".to_string());
            std::process::Command::new("cmd")
                .args(&command.argv[..])
                .spawn()
                .expect("Failed to spawn command")
        }
    };

    let status = child.wait().expect("Failed to exit child");
    if(!status.success()) {
        eprintln!("Process exited with status: {:?}", status.code());
    }
}

pub fn execute_command(command: &mut Command) {
    let Some(program) = command.argv.first() else{
        return;
    };

    match program.as_str() {
        "exit" => std::process::exit(0),
        "cd" => change_dir(&command.argv),
        "echo" => echo_builtin(&command),
        "ls" => list_dir(&command),
        _ => execute_external(command)
    }
}
pub fn execute(pipeline: Pipeline) {
    for mut command in pipeline.commands {
        execute_command(&mut command);
    }
}