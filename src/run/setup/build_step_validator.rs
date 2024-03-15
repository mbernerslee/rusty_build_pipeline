use crate::build_step::BuildStep;
use std::collections::HashMap;

pub fn validate(build_steps: &Vec<BuildStep>) -> Result<(), String> {
    let mut map: HashMap<&String, &BuildStep> = HashMap::new();
    for build_step in build_steps {
        map.insert(&build_step.build_step_name, build_step);
    }
    for build_step in build_steps {
        for dependency in &build_step.depends_on {
            if !map.contains_key(dependency) {
                return Err(format!(
                    "Giving up because build_step_name '{}' depends on '{}', but that doesn't exist!",
                    build_step.build_step_name, dependency
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    mod validate {
        use super::super::*;
        use crate::build_step::BuildStep;
        use crate::build_step::CommandType::Script;

        #[test]
        fn with_no_build_steps_its_ok() {
            assert_eq!(validate(&vec![]), Ok(()))
        }

        #[test]
        fn with_build_steps_that_dont_depend_on_anything_its_ok() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: Script,
                    command: String::from("echo A"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: Script,
                    command: String::from("echo B"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: Script,
                    command: String::from("echo C"),
                    depends_on: vec![],
                    env_vars: None,
                },
            ];
            assert_eq!(validate(&build_steps), Ok(()))
        }

        #[test]
        fn with_build_steps_that_all_depend_on_a_real_one_its_ok() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: Script,
                    command: String::from("echo A"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: Script,
                    command: String::from("echo B"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: Script,
                    command: String::from("echo C"),
                    depends_on: vec![String::from("B")],
                    env_vars: None,
                },
            ];
            assert_eq!(validate(&build_steps), Ok(()))
        }

        #[test]
        fn when_one_build_step_depends_on_a_missing_one_its_not_ok() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: Script,
                    command: String::from("echo A"),
                    depends_on: vec![String::from("MISSING FAKE ONE!")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: Script,
                    command: String::from("echo B"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: Script,
                    command: String::from("echo C"),
                    depends_on: vec![String::from("B")],
                    env_vars: None,
                },
            ];
            assert_eq!(
                validate(&build_steps),
                Err(String::from("Giving up because build_step_name 'A' depends on 'MISSING FAKE ONE!', but that doesn't exist!"))
            )
        }
    }
}
