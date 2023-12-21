//use command_line_arguments::parse;
use std::env;
use std::process;

use crate::command_line_arguments::RunOrInit::*;
use crate::command_line_arguments::*;
mod command_line_arguments;

fn main() {
    //TODO delete jank placeholder tests
    match command_line_arguments::parse(env::args()) {
        Some(CLICommand {
            kind: Run,
            args: run_args,
        }) => {
            println!("run!");
            dbg!(run_args);
        }
        Some(CLICommand {
            kind: Init,
            args: init_args,
        }) => {
            println!("init!");
            dbg!(init_args);
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
