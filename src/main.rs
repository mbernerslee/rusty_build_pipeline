//use command_line_arguments::parse;
use std::env;
use std::process;

use crate::command_line_arguments::CLICommand;
use crate::command_line_arguments::RunOrInit::*;
mod command_line_arguments;

fn main() {
    //TODO delete jank placeholder tests
    match command_line_arguments::parse(env::args()) {
        Some(CLICommand { kind: Run, args: _ }) => {
            println!("run!");
        }
        Some(CLICommand {
            kind: Init,
            args: _,
        }) => {
            println!("init!");
        }
        None => {
            eprintln!("{HELP_MSG}");
            process::exit(1);
        }
    }
}

const HELP_MSG: &'static str = "I only accept arguments of
    - --version
    - run [run args]
    - init [run args]

  If you're unsure of what to do, read the README
  or try running:
    - run help
    - init help
  ";
