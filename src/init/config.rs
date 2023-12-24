pub fn build(args: Vec<String>) -> Result<Config, String> {
    match &args[..] {
        [a, b] => build_with_cwd(a, b),
        [] => Ok(Config {
            working_directory: String::from(DEFAULT_WORKING_DIRECTORY),
        }),
        _ => Err(String::from("Nope")),
    }
}

fn build_with_cwd(flag: &String, working_directory: &String) -> Result<Config, String> {
    if flag.as_str() == CWD_FLAG {
        Ok(Config {
            working_directory: working_directory.clone(),
        })
    } else {
        Err(String::from("Nope"))
    }
}

#[derive(Debug)]
pub struct Config {
    pub working_directory: String,
}

const DEFAULT_WORKING_DIRECTORY: &'static str = ".";
const CWD_FLAG: &'static str = "--cwd";

//#[cfg(test)]
//mod test {
//
//    mod parse {
//        use super::super::RunOrInit::*;
//        use super::super::*;
//
//        #[test]
//        fn given_run_and_more_args_returns_run() {
//            let args = ["ignored_program_name", RUN, "1", "2"];
//            let cli_args = args.iter().map(|s| s.to_string());
//            let expected_run_args = ["1", "2"].map(|s| s.to_string()).to_vec();
//            let result = parse(cli_args);
//            let expected_result = Some(CLICommand {
//                kind: Run,
//                args: expected_run_args,
//            });
//            assert_eq!(result, expected_result);
//        }
//
//        #[test]
//        fn given_init_and_more_args_returns_init() {
//            let args = ["ignored_program_name", INIT, "1", "2"];
//            let cli_args = args.iter().map(|s| s.to_string());
//            let expected_init_args = ["1", "2"].map(|s| s.to_string()).to_vec();
//            let result = parse(cli_args);
//            let expected_result = Some(CLICommand {
//                kind: Init,
//                args: expected_init_args,
//            });
//            assert_eq!(result, expected_result);
//        }
//
//        #[test]
//        fn given_nonsense_returns_none() {
//            let args = ["ignored_program_name", "nonsense"];
//            let cli_args = args.iter().map(|s| s.to_string());
//            let result = parse(cli_args);
//            assert_eq!(result, None);
//
//            let args = ["ignored_program_name", "nonsense", "1", "2"];
//            let cli_args = args.iter().map(|s| s.to_string());
//            let result = parse(cli_args);
//            assert_eq!(result, None);
//
//            let args = ["ignored_program_name"];
//            let cli_args = args.iter().map(|s| s.to_string());
//            let result = parse(cli_args);
//            assert_eq!(result, None);
//        }
//    }
//}
