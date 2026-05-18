use std::env;
use std::path::Path;
use crate::decl::parser::{Command, Pipeline};

fn execute_builtin(program: &str) {
    match program {
        "echo" => {

        },
        "cd" => {

        },
        _ => {}
    }
}

fn change_dir(commands: &Vec<String>) {
    let path = Path::new(commands[1].as_str());

    if!env::set_current_dir(path).is_ok() {
        eprintln!("Invalid path provided");
    }
}

fn execute_external(program: &str) {

}

fn execute_command(command: &Command) {
    let Some(program) = command.argv.first() else{
        return;
    };

    match program.as_str() {
        "exit" => std::process::exit(0),
        "cd" => change_dir(&command.argv),
        "echo" => execute_builtin(program),
        _ => execute_external(program)
    }
}
pub fn execute(pipeline: Pipeline) {
    for command in pipeline.commands {
        execute_command(&command);
    }
}