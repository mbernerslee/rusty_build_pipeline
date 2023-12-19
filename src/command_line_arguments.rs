#[derive(Debug, PartialEq)]
pub enum RunOrInit {
    Run,
    Init,
}

#[derive(Debug)]
pub struct CLICommand {
    pub kind: RunOrInit,
    pub args: Vec<String>,
}

impl PartialEq for CLICommand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.args == other.args
    }
}

pub fn parse(cli_args: Vec<String>) -> Option<CLICommand> {
    let mode = &cli_args[1];
    let mode_args = &cli_args[2..];

    if mode == "run" {
        Some(CLICommand {
            kind: RunOrInit::Run,
            args: mode_args.to_vec(),
        })
    } else if mode == "init" {
        Some(CLICommand {
            kind: RunOrInit::Init,
            args: mode_args.to_vec(),
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
            let cli_args = Vec::from([
                "ignored_program_name".to_string(),
                "run".to_string(),
                "1".to_string(),
                "2".to_string(),
            ]);
            let expected_run_args = Vec::from(["1".to_string(), "2".to_string()]);
            let result = parse(cli_args);
            let expected_result = Some(CLICommand {
                kind: Run,
                args: expected_run_args,
            });
            assert_eq!(result, expected_result);
        }
    }
}
