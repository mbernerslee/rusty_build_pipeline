use std::env;
use std::process;

use crate::command_line_arguments::*;
pub mod build_step;
mod command_line_arguments;
mod init;
mod run;
mod version;

fn main() {
    match command_line_arguments::parse(env::args()) {
        Command::Run(run_args) => match run::main(run_args) {
            Ok(()) => (),
            Err(error) => exit_with_error(error),
        },
        Command::Init(init_args) => match init::main(init_args) {
            Ok(()) => (),
            Err(error) => exit_with_error(error),
        },
        Command::DisplayVersion => version::display(),
        Command::NoValidCommand => {
            exit_with_error(HELP_MSG.to_string());
        }
    }
}

fn exit_with_error(error: String) {
    eprintln!("{error}");
    process::exit(1);
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
