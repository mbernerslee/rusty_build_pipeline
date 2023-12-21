#[derive(Debug, PartialEq)]
pub enum RunOrInit {
    Run,
    Init,
}

#[derive(Debug, PartialEq)]
pub struct CLICommand {
    pub kind: RunOrInit,
    pub args: Vec<String>,
}

pub fn parse<T>(mut cli_args: T) -> Option<CLICommand>
where
    T: Iterator<Item = String>,
{
    cli_args.next();
    let mode = cli_args.next().unwrap_or(String::from("none"));
    let mode_args: Vec<String> = cli_args.collect();

    if mode == RUN {
        Some(CLICommand {
            kind: RunOrInit::Run,
            args: mode_args,
        })
    } else if mode == INIT {
        Some(CLICommand {
            kind: RunOrInit::Init,
            args: mode_args,
        })
    } else {
        None
    }
}

const RUN: &'static str = "run";
const INIT: &'static str = "init";

#[cfg(test)]
mod test {

    mod parse {
        use super::super::RunOrInit::*;
        use super::super::*;

        #[test]
        fn given_run_and_more_args_returns_run() {
            let args = ["ignored_program_name", RUN, "1", "2"];
            let cli_args = args.iter().map(|s| s.to_string());
            let expected_run_args = ["1", "2"].map(|s| s.to_string()).to_vec();
            let result = parse(cli_args);
            let expected_result = Some(CLICommand {
                kind: Run,
                args: expected_run_args,
            });
            assert_eq!(result, expected_result);
        }

        #[test]
        fn given_init_and_more_args_returns_init() {
            let args = ["ignored_program_name", INIT, "1", "2"];
            let cli_args = args.iter().map(|s| s.to_string());
            let expected_init_args = ["1", "2"].map(|s| s.to_string()).to_vec();
            let result = parse(cli_args);
            let expected_result = Some(CLICommand {
                kind: Init,
                args: expected_init_args,
            });
            assert_eq!(result, expected_result);
        }

        #[test]
        fn given_nonsense_returns_none() {
            let args = ["ignored_program_name", "nonsense"];
            let cli_args = args.iter().map(|s| s.to_string());
            let result = parse(cli_args);
            assert_eq!(result, None);

            let args = ["ignored_program_name", "nonsense", "1", "2"];
            let cli_args = args.iter().map(|s| s.to_string());
            let result = parse(cli_args);
            assert_eq!(result, None);

            let args = ["ignored_program_name"];
            let cli_args = args.iter().map(|s| s.to_string());
            let result = parse(cli_args);
            assert_eq!(result, None);
        }
    }
}
