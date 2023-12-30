mod command_line_arguments;
mod config_file;
mod environment_variables;

pub fn determine(args: Vec<String>) -> Result<Setup, String> {
    let mut setup = default();
    let from_failed = environment_variables::read_from_failed()?;
    setup.from_failed = from_failed;
    setup = command_line_arguments::parse(setup, args)?;
    let _raw_config = config_file::read(&setup.cwd)?;
    Ok(setup)
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Verbose,
    Debug,
    AnalyseSelfWorth,
}

#[derive(Debug, PartialEq)]
pub struct Setup {
    pub cwd: String,
    pub mode: Mode,
    pub from_failed: bool,
    pub show_stats: bool,
    pub json_report: bool,
    pub halt_when_done: bool,
}

pub fn default() -> Setup {
    Setup {
        cwd: String::from("."),
        mode: Mode::Normal,
        from_failed: false,
        show_stats: false,
        json_report: false,
        halt_when_done: true,
    }
}

#[cfg(test)]
mod test {
    mod determine {
        use super::super::*;
        use crate::run::setup::command_line_arguments;

        fn build_args(args: &[&str]) -> Vec<String> {
            args.iter().map(|s| s.to_string()).collect()
        }

        #[test]
        fn with_valid_args_retuns_a_setup() {
            let args = build_args(&["--cwd", "example_projects/simple_and_functioning"]);

            let mut expected_setup = default();
            expected_setup.cwd = String::from("example_projects/simple_and_functioning");
            //some odd code to cope with your system setting BUILD_PIPELINE_FROM_FAILED = true or
            //false
            match environment_variables::read_from_failed() {
                Ok(from_failed) => expected_setup.from_failed = from_failed,
                _ => panic!("this test shouldn't get here"),
            }

            assert_eq!(determine(args), Ok(expected_setup))
        }

        #[test]
        fn when_from_failed_is_true_by_env_var_can_be_overriden_to_false() {
            let args = build_args(&["--ra"]);

            let mut expected_setup = default();
            expected_setup.from_failed = false;

            assert_eq!(determine(args), Ok(expected_setup))
        }

        #[test]
        fn errors_when_the_command_line_args_are_jank() {
            let args = build_args(&["--jank"]);
            let error = command_line_arguments::usage();

            assert_eq!(determine(args), Err(error))
        }
    }
}
