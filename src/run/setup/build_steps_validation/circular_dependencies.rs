use crate::build_step::*;
use std::collections::HashMap;

pub fn run(build_steps: &Vec<BuildStep>) -> Result<(), String> {
    let (names, deps) = deps(build_steps);
    check_circular_deps(names, deps);
    Ok(())
}

fn deps(build_steps: &Vec<BuildStep>) -> (Vec<&String>, HashMap<&String, Vec<&String>>) {
    let mut tree: HashMap<&String, Vec<&String>> = HashMap::new();
    let mut names: Vec<&String> = Vec::new();

    for build_step in build_steps {
        let mut deps: Vec<&String> = Vec::new();

        for dependent in &build_step.depends_on {
            deps.push(&dependent);
        }

        tree.insert(&build_step.build_step_name, deps);
        names.push(&build_step.build_step_name);
    }

    (names, tree)
}

fn check_circular_deps(
    names: Vec<&String>,
    deps: HashMap<&String, Vec<&String>>,
) -> Result<(), String> {
    Ok(())
}

fn error() -> Result<(), String> {
    Err(String::from("Giving up because the config.json was invalid. I found a circular dependency! \nAt least one 'depends_on' eventually depends upon itself, meaning that the build_pipeline can never finished. Fix it"))
}

#[cfg(test)]
mod test {
    mod run {
        use super::super::*;

        #[test]
        fn error_if_a_step_depends_on_itself() {
            let build_steps = vec![BuildStep {
                build_step_name: String::from("A"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![String::from("A")],
                env_vars: None,
            }];

            assert_eq!(run(&build_steps), error())
        }

        #[test]
        fn ok_if_a_step_depends_on_nothing() {
            let build_steps = vec![BuildStep {
                build_step_name: String::from("A"),
                command_type: CommandType::ShellCommand,
                command: String::from("echo 'hello'"),
                depends_on: vec![],
                env_vars: None,
            }];

            assert_eq!(run(&build_steps), Ok(()))
        }
    }
}
