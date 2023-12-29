use std::env;
use std::process;

use crate::command_line_arguments::*;
mod command_line_arguments;
mod exit;
mod init;

fn main() {
    match command_line_arguments::parse(env::args()) {
        Command::Run(run_args) => {
            println!("run!");
            dbg!(run_args);
        }
        Command::Init(init_args) => match init::main(init_args) {
            Ok(()) => (),
            Err(error) => exit::with_error(error),
        },
        Command::NoValidCommand => {
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

// no tests here because testing at this level is covered by end to end tests
