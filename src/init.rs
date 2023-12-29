use crate::init::directory_structure::*;
use config::Config;

mod config;
mod directory_structure;
mod directory_structure_creation;

pub fn main(args: Vec<String>) -> Result<(), String> {
    create_dir_structure(config::build(args))
}

fn create_dir_structure(maybe_config: Result<Config, String>) -> Result<(), String> {
    match maybe_config {
        Ok(config) => create_directory_structure(&config),
        Err(error) => Err(error),
    }
}

fn create_directory_structure(config: &Config) -> Result<(), String> {
    let directory_structure = directory_structure::determine(config);
    match directory_structure_creation::create(&directory_structure) {
        Ok(()) => {
            print_success(&directory_structure);
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn print_success(d: &DirectoryStructure) {
    println!(
        "Successful initialised!

{0}/
└── {1}
    ├── {2}
    └── {3}",
        d.working_directory_path.to_str().unwrap(),
        BUILD_PIPELINE,
        CONFIG_JSON,
        SCRIPTS
    )
}

#[cfg(test)]
mod test {

    mod main {
        use super::super::*;
        use std::path::PathBuf;

        // this test creates directories and files in a gitignored directory
        #[test]
        fn works_given_a_working_directory_that_exists() {
            let cwd = "example_projects/test_project/0";
            let _ = std::fs::remove_dir_all(cwd);
            let _ = std::fs::create_dir_all(cwd);
            let args = ["--cwd", cwd].iter().map(|s| s.to_string()).collect();

            assert_eq!(Ok(()), main(args));

            assert_eq!(
                PathBuf::from(format!("{cwd}/build_pipeline")).exists(),
                true
            );

            assert_eq!(
                PathBuf::from(format!("{cwd}/build_pipeline/scripts")).exists(),
                true
            );

            assert_eq!(
                PathBuf::from(format!("{cwd}/build_pipeline/config.json")).exists(),
                true
            );

            let _ = std::fs::remove_dir_all(cwd);
        }

        #[test]
        fn errors_given_bad_args() {
            let args = ["stupid", "args"].iter().map(|s| s.to_string()).collect();
            match main(args) {
                Err(_error) => assert!(true),
                _ => panic!("This test should return error"),
            }
        }

        #[test]
        fn errors_when_directory_structure_creation_fails() {
            let args = ["--cwd", "bad"].iter().map(|s| s.to_string()).collect();
            match main(args) {
                Err(_error) => assert!(true),
                _ => panic!("This test should return error"),
            }
        }
    }
}
