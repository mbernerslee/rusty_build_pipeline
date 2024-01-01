use crate::build_step::*;
use std::collections::{HashMap, HashSet};

pub fn run(build_steps: &Vec<BuildStep>) -> Result<(), String> {
    let (names, deps) = deps(build_steps);
    check_circular_deps(names, deps)
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
    all_deps: HashMap<&String, Vec<&String>>,
) -> Result<(), String> {
    for name in names {
        let my_deps: HashSet<&String> = HashSet::new();
        match check_circular_deps_from(name, my_deps, &all_deps) {
            Ok(()) => (),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn check_circular_deps_from<'a>(
    name: &String,
    mut my_deps: HashSet<&'a String>,
    all_deps: &HashMap<&String, Vec<&'a String>>,
) -> Result<(), String> {
    match all_deps.get(name) {
        Some(deps) => {
            for dep in deps {
                if my_deps.insert(dep) {
                    return check_circular_deps_from(dep, my_deps, all_deps);
                } else {
                    return error();
                }
            }
        }
        None => return error(),
    }
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
        fn error_if_a_step_eventually_depends_on_itself() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("B"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("C")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("C"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("E")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("E"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
            ];

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

        #[test]
        fn ok_if_steps_have_many_dependencies_but_none_are_circular() {
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
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("E"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("F"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("E")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("G"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("H"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("G")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("I"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("H")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn ok_if_steps_have_many_branching_dependencies_but_none_are_circular() {
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
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B"), String::from("C")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("E"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("F"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("G"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("E"), String::from("F")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("H"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D"), String::from("G")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), Ok(()))
        }

        #[test]
        fn error_if_steps_have_many_branching_dependencies_and_one_is_circular() {
            let build_steps = vec![
                BuildStep {
                    build_step_name: String::from("A"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("H")],
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
                    depends_on: vec![String::from("A")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("D"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("B"), String::from("C")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("E"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("F"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("G"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("E"), String::from("F")],
                    env_vars: None,
                },
                BuildStep {
                    build_step_name: String::from("H"),
                    command_type: CommandType::ShellCommand,
                    command: String::from("echo 'hello'"),
                    depends_on: vec![String::from("D"), String::from("G")],
                    env_vars: None,
                },
            ];

            assert_eq!(run(&build_steps), error())
        }
    }
}
