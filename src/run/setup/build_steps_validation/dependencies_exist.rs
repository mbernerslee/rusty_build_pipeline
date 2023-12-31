use crate::build_step::*;
use std::collections::HashSet;

pub fn run(build_steps: &Vec<BuildStep>) -> Result<(), String> {
    if all_dependencies_exist(&build_steps) {
        Ok(())
    } else {
        error()
    }
}

//TODO collapse this & unqiue_name into one, for better efficiency. Could easily merge
fn all_dependencies_exist(build_steps: &Vec<BuildStep>) -> bool {
    let mut names: HashSet<String> = HashSet::new();

    for build_step in build_steps {
        names.insert(build_step.build_step_name.clone());
    }

    for build_step in build_steps {
        for dependent in &build_step.depends_on {
            if !names.contains(dependent) {
                return false;
            }
        }
    }
    true
}

fn error() -> Result<(), String> {
    Err(String::from("Giving up because the config.json was invalid. It has at least one 'depends_on' which doesn't exist. All depends_on must be lists of build_step_names which exist"))
}

#[cfg(test)]
mod test {
    mod run {
        use super::super::*;

        #[test]
        fn when_all_depends_on_exist_its_ok() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn when_there_are_no_depends_on_its_ok() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn when_a_depends_on_does_not_exist_its_an_error() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("FAKE_NONESENSE")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), error())
        }

        #[test]
        fn when_many_depends_on_do_not_exist_its_an_error() {
            let build_steps = vec![BuildStep {
                build_step_name: String::from("A"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![String::from("NOPE"), String::from("FAKE")],
                env_vars: None,
            }];

            assert_eq!(run(&build_steps), error())
        }
    }
}
