#[derive(Debug, PartialEq)]
pub enum Command {
    Run(Vec<String>),
    Init(Vec<String>),
    DisplayVersion,
    NoValidCommand,
}

pub fn parse<T>(mut cli_args: T) -> Command
where
    T: Iterator<Item = String>,
{
    cli_args.next();
    let mode = cli_args.next().unwrap_or(String::from("none"));
    let mode_args: Vec<String> = cli_args.collect();

    if mode == RUN {
        Command::Run(mode_args)
    } else if mode == INIT {
        Command::Init(mode_args)
    } else if mode == VERSION {
        Command::DisplayVersion
    } else {
        Command::NoValidCommand
    }
}

const RUN: &'static str = "run";
const INIT: &'static str = "init";
const VERSION: &'static str = "--version";

#[cfg(test)]
mod test {

    mod parse {
        use super::super::*;

        #[test]
        fn given_run_and_more_args_returns_run() {
            let args = ["ignored_program_name", RUN, "1", "2"];
            let cli_args = args.iter().map(|s| s.to_string());
            let expected_run_args = ["1", "2"].map(|s| s.to_string()).to_vec();
            let result = parse(cli_args);
            let expected_result = Command::Run(expected_run_args);
            assert_eq!(result, expected_result);
        }

        #[test]
        fn given_init_and_more_args_returns_init() {
            let args = ["ignored_program_name", INIT, "1", "2"];
            let cli_args = args.iter().map(|s| s.to_string());
            let expected_init_args = ["1", "2"].map(|s| s.to_string()).to_vec();
            let result = parse(cli_args);
            let expected_result = Command::Init(expected_init_args);
            assert_eq!(result, expected_result);
        }

        #[test]
        fn given_version_returns_display_version() {
            let args = ["ignored_program_name", "--version"];
            let cli_args = args.iter().map(|s| s.to_string());
            let result = parse(cli_args);
            assert_eq!(result, Command::DisplayVersion);
        }

        #[test]
        fn given_nonsense_returns_none() {
            let args = ["ignored_program_name", "nonsense"];
            let cli_args = args.iter().map(|s| s.to_string());
            let result = parse(cli_args);
            assert_eq!(result, Command::NoValidCommand);

            let args = ["ignored_program_name", "nonsense", "1", "2"];
            let cli_args = args.iter().map(|s| s.to_string());
            let result = parse(cli_args);
            assert_eq!(result, Command::NoValidCommand);

            let args = ["ignored_program_name"];
            let cli_args = args.iter().map(|s| s.to_string());
            let result = parse(cli_args);
            assert_eq!(result, Command::NoValidCommand);
        }
    }
}
