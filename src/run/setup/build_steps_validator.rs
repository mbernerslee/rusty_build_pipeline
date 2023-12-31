use crate::build_step::*;
use std::collections::HashSet;

pub fn validate(build_steps: &Vec<BuildStep>) -> Result<(), String> {
    if dbg!(names_are_unique(build_steps)) {
        Ok(())
    } else {
        error()
    }
}

fn names_are_unique(build_steps: &Vec<BuildStep>) -> bool {
    let mut names: HashSet<String> = HashSet::new();
    for step in build_steps {
        if !names.insert(step.build_step_name.clone()) {
            return false;
        }
    }
    true
}

fn error() -> Result<(), String> {
    Err(String::from("Giving up because the config.json was invalid. It had duplicate build_step_names, but they must all be unique"))
}

#[cfg(test)]
mod test {
    mod determine_build_steps {
        use super::super::*;

        #[test]
        fn when_build_step_names_are_unique_its_ok() {
            let build_steps = vec![BuildStep {
                build_step_name: String::from("sayHello"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![],
                env_vars: None,
            }];

            assert_eq!(validate(&build_steps), Ok(()))
        }

        #[test]
        fn when_build_step_names_are_not_unique_return_error() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("sayHello"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("sayHello"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
            ];

            assert_eq!(validate(&build_steps), error())
        }
    }
}
