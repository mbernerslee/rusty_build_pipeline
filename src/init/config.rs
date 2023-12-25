pub fn build(args: Vec<String>) -> Result<Config, String> {
    match &args[..] {
        [a, b] => build_with_cwd(a, b),
        [] => Ok(Config {
            working_directory: String::from(DEFAULT_WORKING_DIRECTORY),
        }),
        _ => error(),
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub working_directory: String,
}

fn build_with_cwd(flag: &String, working_directory: &String) -> Result<Config, String> {
    if flag.as_str() == CWD_FLAG {
        Ok(Config {
            working_directory: working_directory.clone(),
        })
    } else {
        error()
    }
}

fn error() -> Result<Config, String> {
    Err(String::from(USAGE))
}

const DEFAULT_WORKING_DIRECTORY: &'static str = ".";
const CWD_FLAG: &'static str = "--cwd";
const USAGE: &'static str = "I only accept arguements of:

--cwd path/to/dir (optional)
  sets the directory in which to initialise build_pipeline. Defaults to the current directory
";

#[cfg(test)]
mod test {

    mod build {
        use super::super::*;

        #[test]
        fn given_cwd_and_a_dir_only_returns_a_config() {
            let args = ["--cwd", "path/to/some/dir"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            let result = build(args);
            let expected_result = Ok(Config {
                working_directory: String::from("path/to/some/dir"),
            });
            assert_eq!(result, expected_result);
        }

        #[test]
        fn given_cwd_and_a_dir_but_with_extra_args_returns_error() {
            let args = ["--cwd", "path/to/some/dir", "fail", "oops"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            let result = build(args);
            let expected_result = Err(String::from(USAGE));
            assert_eq!(result, expected_result);
        }

        #[test]
        fn given_two_nonsense_args_returns_error() {
            let args = ["--nope", "nopey"].iter().map(|s| s.to_string()).collect();
            let result = build(args);
            let expected_result = Err(String::from(USAGE));
            assert_eq!(result, expected_result);
        }

        #[test]
        fn given_nonsense_returns_error() {
            let args = ["total", "nonsense", "wow", "fail"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            let result = build(args);
            let expected_result = Err(String::from(USAGE));
            assert_eq!(result, expected_result);
        }
    }
}
