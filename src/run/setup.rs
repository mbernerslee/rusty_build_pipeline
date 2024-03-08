use crate::build_step::BuildStep;
use std::collections::{HashMap, HashSet};

mod command_line_arguments;
mod config_file;
mod config_file_json_parser;
mod environment_variables;
mod terminal_width;
mod tree;

pub fn determine(args: Vec<String>) -> Result<(Setup, Vec<BuildStep>), String> {
    let mut setup = default();
    let from_failed = environment_variables::read_from_failed()?;
    setup.from_failed = from_failed;
    setup = command_line_arguments::parse(setup, args)?;
    let raw_config = config_file::read(&setup.cwd)?;
    let build_steps = config_file_json_parser::determine_build_steps(&raw_config)?;
    //let _tree = tree::build(&build_steps)?; // TODO
    setup = terminal_width::add_to_setup(setup)?;
    Ok((setup, build_steps))
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Verbose,
    Debug,
    AnalyseSelfWorth,
}

pub type Tree<'a> = HashMap<&'a str, HashSet<&'a str>>;

#[derive(Debug, PartialEq)]
pub struct Setup {
    pub cwd: String,
    pub mode: Mode,
    pub from_failed: bool,
    pub show_stats: bool,
    pub json_report: bool,
    pub halt_when_done: bool,
    pub terminal_width: u16,
}

pub fn default() -> Setup {
    Setup {
        cwd: String::from("."),
        mode: Mode::Normal,
        from_failed: false,
        show_stats: false,
        json_report: false,
        halt_when_done: true,
        terminal_width: 80,
    }
}

//#[cfg(test)]
//mod test {
//    mod determine {
//        use super::super::*;
//        use crate::run::setup::command_line_arguments;
//
//        fn build_args(args: &[&str]) -> Vec<String> {
//            args.iter().map(|s| s.to_string()).collect()
//        }
//
//        fn simple_yet_functioning_build_steps() -> Vec<BuildStep> {
//            let file_path = String::from("example_projects/simple_and_functioning");
//            let config_file = config_file::read(&file_path).unwrap();
//            config_file_json_parser::determine_build_steps(&config_file).unwrap()
//        }
//
//        #[test]
//        fn with_valid_args_retuns_a_setup() {
//            let args = build_args(&["--cwd", "example_projects/simple_and_functioning"]);
//
//            let mut expected_setup = default();
//            expected_setup.cwd = String::from("example_projects/simple_and_functioning");
//            //some odd code to cope with your system setting BUILD_PIPELINE_FROM_FAILED = true or
//            //false
//            match environment_variables::read_from_failed() {
//                Ok(from_failed) => expected_setup.from_failed = from_failed,
//                _ => panic!("this test shouldn't get here"),
//            }
//
//            assert_eq!(
//                determine(args),
//                Ok((expected_setup, simple_yet_functioning_build_steps()))
//            )
//        }
//
//        #[test]
//        fn when_from_failed_is_true_by_env_var_can_be_overriden_to_false() {
//            let args = build_args(&["--ra", "--cwd", "example_projects/simple_and_functioning"]);
//
//            let mut expected_setup = default();
//            expected_setup.from_failed = false;
//            expected_setup.cwd = String::from("example_projects/simple_and_functioning");
//
//            assert_eq!(
//                determine(args),
//                Ok((expected_setup, simple_yet_functioning_build_steps()))
//            )
//        }
//
//        #[test]
//        fn errors_when_the_command_line_args_are_jank() {
//            let args = build_args(&["--jank"]);
//            let error = command_line_arguments::usage();
//
//            assert_eq!(determine(args), Err(error))
//        }
//
//        #[test]
//        fn errors_when_config_json_file_is_jank() {
//            let args = build_args(&["--cwd", "example_projects/bad_json"]);
//            let error = String::from("Failed to parse the config.json file.\nSee more specific parsing error:\n\ninvalid type: map, expected a sequence at line 1 column 0\n");
//
//            assert_eq!(determine(args), Err(error))
//        }
//
//        #[test]
//        fn errors_when_the_config_is_valid_json_but_invalid_for_another_reason() {
//            let args = build_args(&["--cwd", "example_projects/bad_config"]);
//            let error = String::from("Giving up because the config.json was invalid. It has at least one 'depends_on' which doesn't exist. All depends_on must be lists of build_step_names which exist");
//
//            assert_eq!(determine(args), Err(error))
//        }
//    }
//}
