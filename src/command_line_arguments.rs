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
    let mode = cli_args.next();
    let mode_args: Vec<String> = cli_args.collect();

    if mode == Some(String::from("run")) {
        Some(CLICommand {
            kind: RunOrInit::Run,
            args: mode_args,
        })
    } else if mode == Some(String::from("init")) {
        Some(CLICommand {
            kind: RunOrInit::Init,
            args: mode_args,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod test {

    mod parse {
        use super::super::RunOrInit::*;
        use super::super::*;

        #[test]
        fn given_run_and_more_args_returns_run() {
            let args = ["ignored_program_name", "run", "1", "2"];
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
            let args = ["ignored_program_name", "init", "1", "2"];
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
