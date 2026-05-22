use std::{env, io, fs};
use std::path::{Path, PathBuf};
use is_executable::is_executable;
use walkdir::WalkDir;
use log::log;
use crate::decl::parser::Command;

pub fn resolve_path(program: &str) -> Option<PathBuf> {
    if program.contains('\\') {
        let path = PathBuf::from(program);
        if path.exists() {
            return Some(path);
        }else{
            return None;
        }
    }
    let extensions = env::var("PATHEXT").unwrap_or(".EXE;.BAT;.CMD;.COM".to_string());
    let path_var = env::var("PATH").unwrap_or_default();
    // if let Some(path_var) = env::var_os("PATH"){
    //     for path in env::split_paths(&path_var){
    //         for ext in extensions.split(";") {
    //             let full_path = path.join(format!("{}{}", program, ext));
    //             if is_executable(&full_path) || full_path.is_file() {
    //                 return Some(path);
    //             }
    //         }
    //     }
    // }
    for dir in path_var.split(";") {
        for ext in extensions.split(";") {
            let candidate = PathBuf::from(dir)
                .join(format!("{}{}", program, ext));
            if candidate.exists() {
                println!("DEBUG: {:?}", candidate);
                return Some(candidate);
            }
        }
    }
    None
}

pub fn list_dir(command: &Command) {
    let curr_dir = env::current_dir();
    if let Ok(path_buf) = curr_dir {
        if command.argv.len() > 1 {
            if command.argv[1].parse::<usize>().is_ok(){
                let depth = command.argv[1].parse::<usize>().unwrap();
                for entry in WalkDir::new(&path_buf).
                    min_depth(1).
                    max_depth(depth).
                    into_iter().
                    filter_map(|x| x.ok())
                {
                    println!("{}", entry.path().strip_prefix(&path_buf).unwrap().display());
                }
            }else if command.argv[1] == "-r" {
                for entry in WalkDir::new(&path_buf).
                    min_depth(1).
                    into_iter().
                    filter_map(|x| x.ok())
                {
                    println!("{}", entry.path().strip_prefix(&path_buf).unwrap().display());
                }
            }
        }
        for entry in WalkDir::new(&path_buf).
            min_depth(1).
            max_depth(1).
            into_iter().
            filter_map(|x| x.ok())
        {
            println!("{}", entry.path().strip_prefix(&path_buf).unwrap().display());
        }
    };
}